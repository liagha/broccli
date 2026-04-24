use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

pub enum Item {
    Plain(Expr),
    Colored(Expr, Expr),
}

pub struct Items {
    pub list: Vec<Item>,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value = input.parse::<Expr>()?;

        if input.peek(Token![=>]) {
            input.parse::<Token![=>]>()?;
            let color = input.parse::<Expr>()?;
            Ok(Item::Colored(value, color))
        } else {
            Ok(Item::Plain(value))
        }
    }
}

impl Parse for Items {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut list = Vec::new();

        while !input.is_empty() {
            list.push(input.parse()?);

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Items { list })
    }
}
