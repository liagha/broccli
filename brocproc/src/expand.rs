use crate::parse::{Item, Items};
use proc_macro2::TokenStream;
use quote::quote;

pub fn build(items: &Items) -> TokenStream {
    if items.list.is_empty() {
        return quote! { String::new() };
    }

    let parts = items.list.iter().map(|item| match item {
        Item::Plain(value) => {
            quote! {
                format!("{}", #value)
            }
        }
        Item::Colored(value, color) => {
            quote! {
                format!("{}", broccolor::ColoredText {
                    content: #value,
                    color: #color,
                })
            }
        }
    });

    quote! {
        [#(#parts),*].concat()
    }
}
