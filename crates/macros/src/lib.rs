use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn context(attr: TokenStream, item: TokenStream) -> TokenStream {
    // let item = parse_macro
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}
