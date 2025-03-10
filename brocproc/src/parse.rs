use syn::{Expr, Token};
use syn::parse::{Parse, ParseStream};
use syn::token::FatArrow;

#[derive(Clone)]
pub enum MacroArg {
    Simple(Expr),
    KeyValue(Expr, Expr),
}

pub struct MacroArgs {
    pub args: Vec<MacroArg>,
}

impl Parse for MacroArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse::<Expr>()?;

        if input.peek(FatArrow) {
            input.parse::<FatArrow>()?;
            let value = input.parse::<Expr>()?;
            Ok(MacroArg::KeyValue(expr, value))
        } else {
            Ok(MacroArg::Simple(expr))
        }
    }
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Vec::new();

        while !input.is_empty() {
            args.push(input.parse()?);

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(MacroArgs { args })
    }
}
