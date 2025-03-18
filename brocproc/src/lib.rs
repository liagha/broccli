mod parse;
mod xformat;
use quote::quote;
use syn::parse_macro_input;
use xformat::*;
use crate::parse::MacroArgs;

#[proc_macro]
pub fn xprintln(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MacroArgs { args } = parse_macro_input!(input as MacroArgs);
    let out : proc_macro2::TokenStream = xformat_args(&args).into();

    quote! {
        println!("{}", #out)
    }.into()
}

#[proc_macro]
pub fn xprint(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MacroArgs { args } = parse_macro_input!(input as MacroArgs);
    let out : proc_macro2::TokenStream = xformat_args(&args).into();

    quote! {
        print!("{}", #out)
    }.into()
}

#[proc_macro]
pub fn xeprintln(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MacroArgs { args } = parse_macro_input!(input as MacroArgs);
    let out : proc_macro2::TokenStream = xformat_args(&args).into();


    quote! {{
        use broccolor::TextStyle;

        println!("{} {}", "error:".term_colorize(Color::Red), #out)
    }}.into()
}

#[proc_macro]
pub fn xprintb(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MacroArgs { args } = parse_macro_input!(input as MacroArgs);
    let out : proc_macro2::TokenStream = xformat_block(&args, 0).into();

    quote! {
        println!("{}", #out)
    }.into()
}