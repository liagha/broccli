mod parse;
mod xformat;
use xformat::*;

use crate::parse::{MacroArgs};
use syn::parse_macro_input;

#[proc_macro]
pub fn test_proc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MacroArgs { args } = parse_macro_input!(input as MacroArgs);

    let out = xformat_block(&args, 0);

    out
}
