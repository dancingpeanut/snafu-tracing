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

#[proc_macro]
pub fn quick_tracing(input: TokenStream) -> TokenStream {
    errors::quick_tracing(input)
}
