use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Parser};
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericArgument, Ident, Path, PathArguments, Token, Type};

fn extract_type_from_box(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };
    if type_path.path.segments.first()?.ident != "Box" {
        return None;
    }
    let arguments = &type_path.path.segments.first()?.arguments;
    let PathArguments::AngleBracketed(angle_bracketed) = arguments else {
        return None;
    };
    let generic_arg = angle_bracketed.args.first()?;
    let GenericArgument::Type(ty) = generic_arg else {
        return None;
    };
    if matches!(ty, Type::TraitObject(_)) {
        None
    } else {
        Some(ty)
    }
}

pub fn trace_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);
    let syn::Data::Enum(enum_data) = &mut input.data else {
        panic!("not an enum")
    };
    for variant in enum_data.variants.iter_mut() {
        if matches!(variant.fields, syn::Fields::Unit) {
            variant.fields =
                syn::Fields::Named(syn::FieldsNamed::parse.parse2(quote! {{}}).unwrap());
        }
        let syn::Fields::Named(field) = &mut variant.fields else {
            panic!("not a named field ")
        };
        field.named.push(
            syn::Field::parse_named
                .parse2(quote! {#[snafu(implicit)] _location: ::snafu::Location})
                .unwrap(),
        );
        if let Some(source) = field.named.iter_mut().find(|f| {
            let name = f.ident.as_ref().unwrap();
            name == "source" || name == "error"
        }) {
            if let Some(inner_type) = extract_type_from_box(&source.ty) {
                source
                    .attrs
                    .push(parse_quote! {#[snafu(source(from(#inner_type, Box::new)))]})
            } else {
                source.attrs.push(parse_quote! {#[snafu(source)]})
            }
        }
    }

    quote! { #input }.into()
}

pub fn derive_debug_trace(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let syn::Data::Enum(enum_data) = &mut input.data else {
        panic!("not an enum")
    };
    let mut debug_trace_arms = vec![];
    for variant in enum_data.variants.iter_mut() {
        let syn::Fields::Named(field) = &mut variant.fields else {
            panic!("not a named field ")
        };
        let mut cfg_attrs = vec![];
        for attr in &variant.attrs {
            if attr.path().is_ident("cfg") {
                cfg_attrs.push(attr);
            }
        }
        let is_source = |f: &syn::Field| f.ident.as_ref().unwrap() == "source";
        let has_source = field.named.iter().any(is_source);
        let is_error = |f: &syn::Field| f.ident.as_ref().unwrap() == "error";
        let has_error = field.named.iter().any(is_error);

        let variant_name = &variant.ident;
        let debug_trace_arm = if has_source {
            quote! {
                #(#cfg_attrs)*
                #name::#variant_name {_location, source, ..} => {
                    let level = source.debug_trace(f)?;
                    writeln!(f, "{level}: {self}, at {_location}")?;
                    Ok(level + 1)
                }
            }
        } else if has_error {
            quote! {
                #(#cfg_attrs)*
                #name::#variant_name {_location, error, ..} => {
                    writeln!(f, "0: {error}")?;
                    writeln!(f, "1: {self}, at {_location}")?;
                    Ok(2)
                }
            }
        } else {
            quote! {
                #(#cfg_attrs)*
                #name::#variant_name {_location, .. } => {
                    writeln!(f, "0: {self}, at {_location}")?;
                    Ok(1)
                }
            }
        };
        debug_trace_arms.push(debug_trace_arm);
    }

    quote! {
        impl DebugTrace for #name {
            #[inline(never)]
            fn debug_trace(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<u32, ::std::fmt::Error> {
                match self {
                    #(#debug_trace_arms)*
                }
            }
        }

        impl ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                writeln!(f, "{self}")?;
                DebugTrace::debug_trace(self, f)?;
                Ok(())
            }
        }
    }
    .into()
}

struct MacroArgs {
    macro_name: Ident,
    struct_path: Path,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let macro_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let struct_path = input.parse()?;
        if !input.is_empty() {
            return Err(input.error("expected only two arguments"));
        }
        Ok(Self { macro_name, struct_path })
    }
}

pub fn quick_tracing(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as MacroArgs);
    let macro_name = args.macro_name;
    let struct_path = args.struct_path;

    let expanded = quote! {
        #[macro_export]
        macro_rules! #macro_name {
            ($msg:literal) => {
                {
                    #struct_path {
                        _error: $msg.to_string()
                    }.build()
                }
            };
            ($fmt:expr, $($arg:tt)*) => {
                {
                    #struct_path {
                        _error: ::std::format!($fmt, $($arg)*)
                    }.build()
                }
            };
            ($error:expr) => {
                {
                    #struct_path {
                        _error: $error.into()
                    }.build()
                }
            };
        }
    };

    expanded.into()
}

