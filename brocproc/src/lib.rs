mod parse;

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, parse::Parse, Expr};
use crate::parse::{MacroArg, MacroArgs};

#[proc_macro]
pub fn xformat_args(input: TokenStream) -> TokenStream {
    let MacroArgs { mut args } = parse_macro_input!(input as MacroArgs);

    if args.is_empty() {
        return quote! {
            println!("")
        }.into()
    }

    let (format, format_color) =
        match args.remove(0) {
            MacroArg::Simple(arg) => {
                let color: Expr = syn::parse_quote! { Color::White };

                (arg, color)
            }
            MacroArg::KeyValue(arg, color) => {
                (arg, color)
            }
            _ => unimplemented!()
        };

    let mut arg_expressions = Vec::new();

    for arg in args {
        match arg {
            MacroArg::Simple(expr) => {
                arg_expressions.push(quote! { #expr });
            }
            MacroArg::KeyValue(expr, color) => {
                let expression = quote! {
                    broccolor::ColoredText {
                        content: #expr,
                        color: #color
                    }
                };

                arg_expressions.push(expression);
            }
            _ => unimplemented!()
        }
    }

    let result = quote! {
        broccolor::TextStyle::colorize(
            &format!(#format, #(#arg_expressions),*),
            #format_color
        )
    };

    result.into()
}

#[proc_macro]
pub fn tokenize(input: TokenStream) -> TokenStream {
    let MacroArgs { args } = parse_macro_input!(input as MacroArgs);

    let arg_handlers = args.iter().enumerate().map(|(i, arg)| {
        let index = i;

        match arg {
            MacroArg::Simple(expr) => {
                quote! {
                    println!("Argument {}: {:?}", #index, #expr);
                }
            },
            MacroArg::KeyValue(key, value) => {
                quote! {
                    println!("Argument {}: {:?} => {:?}", #index, #key, #value);

                    println!("  - Key: {:?}", #key);

                    println!("  - Value: {:?}", #value);
                }
            }
        }
    });

    let args_len = args.len();
    let result = quote! {
        {
            println!("Processing {} arguments", #args_len);
            #(#arg_handlers)*
            println!("Done processing arguments");

            #args_len
        }
    };

    result.into()
}

