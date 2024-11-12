use crayfish_syn::Context;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn context(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let context = parse_macro_input!(item as Context);

    TokenStream::from(context.into_token_stream())
}
