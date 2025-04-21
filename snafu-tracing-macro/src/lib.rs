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
