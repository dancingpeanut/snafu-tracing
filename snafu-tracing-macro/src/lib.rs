mod errors;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn trace_error(attr: TokenStream, item: TokenStream) -> TokenStream {
    errors::trace_error(attr, item)
}

#[proc_macro_derive(DebugTrace)]
pub fn derive_debug_trace(input: TokenStream) -> TokenStream {
    errors::derive_debug_trace(input)
}

#[proc_macro_attribute]
pub fn wrap_result_ext(_attr: TokenStream, item: TokenStream) -> TokenStream {
    errors::wrap_result_ext(_attr, item) 
}

#[proc_macro_attribute]
pub fn drive_anyerr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    errors::anyerr(_attr, item)
}

use quote::{quote};
use syn::{parse_macro_input, Fields, ItemEnum};

#[proc_macro_attribute]
pub fn enrich_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_enum = parse_macro_input!(item as ItemEnum);
    let enum_ident = &input_enum.ident;

    // 构造要添加的两个字段
    let extra_fields = quote! {
        location: Location,
        chain: Option<Box<#enum_ident>>
    };

    // 修改已有变体
    for variant in &mut input_enum.variants {
        if let Fields::Named(fields_named) = &mut variant.fields {
            let parsed: syn::FieldsNamed = syn::parse2(quote!({ #extra_fields })).unwrap();
            fields_named.named.extend(parsed.named);
        } else {
            return syn::Error::new_spanned(
                &variant,
                "Only struct-like enum variants are supported",
            ).to_compile_error().into();
        }
    }

    // 添加新的两个变体
    let extra_variants: syn::ItemEnum = syn::parse2(quote! {
        enum Dummy {
            Message {
                msg: String,
                #extra_fields
            },
            Wrap {
                error: Box<dyn std::error::Error + Send + Sync + 'static>,
                #extra_fields
            }
        }
    }).unwrap();

    input_enum.variants.extend(extra_variants.variants);

    let output = quote! {
        #input_enum
    };

    output.into()
}

#[proc_macro_attribute]
pub fn enrich_with_chain(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_enum = parse_macro_input!(item as ItemEnum);
    let enum_ident = &input_enum.ident;

    // 提取变体名用于生成 match 分支
    let mut match_arms = vec![];

    for variant in &input_enum.variants {
        let var_ident = &variant.ident;

        if let Fields::Named(fields_named) = &variant.fields {
            let has_chain = fields_named.named.iter().any(|f| f.ident.as_ref().unwrap() == "chain");

            if has_chain {
                match_arms.push(quote! {
                    Self::#var_ident { chain: c, .. } => *c = Some(Box::new(self)),
                });
            }
        }
    }

    let enum_def = quote! {
        #input_enum

        impl #enum_ident {
            pub fn with_chain(self, mut chain: Self) -> Self {
                match &mut chain {
                    #(#match_arms)*
                }
                chain
            }
        }
    };

    enum_def.into()
}