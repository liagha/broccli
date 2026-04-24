mod expand;
mod parse;

use expand::build;
use parse::Items;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn xprint(input: TokenStream) -> TokenStream {
    let items = parse_macro_input!(input as Items);
    let text = build(&items);

    quote! {
        print!("{}", #text)
    }
        .into()
}

#[proc_macro]
pub fn xprintln(input: TokenStream) -> TokenStream {
    let items = parse_macro_input!(input as Items);
    let text = build(&items);

    quote! {
        println!("{}", #text)
    }
        .into()
}

#[proc_macro]
pub fn xeprintln(input: TokenStream) -> TokenStream {
    let items = parse_macro_input!(input as Items);
    let text = build(&items);

    quote! {
        eprintln!("{}", #text)
    }
        .into()
}
