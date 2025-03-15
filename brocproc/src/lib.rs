mod parse;
use crate::parse::{MacroArg, MacroArgs};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn test_proc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let MacroArgs { args } = parse_macro_input!(input as MacroArgs);

    let out = xformat_block(&args, 0);

    out
}

fn xformat_args(args: &Vec<MacroArg>) -> proc_macro::TokenStream {
    fn xformat_arg(arg: &MacroArg) -> proc_macro::TokenStream {
        match &arg {
            MacroArg::Simple(expr) => quote! {
                format!("{}", #expr)
            }
            .into(),
            MacroArg::KeyValue(expr, color) => quote! {
                format!(
                    "{}",
                    broccolor::ColoredText {
                        content: #expr,
                        color: #color,
                    }
                )
            }
            .into(),
            MacroArg::Group(inner_args) => {
                let inner_tokens: proc_macro2::TokenStream = xformat_args(inner_args).into();
                quote! { #inner_tokens }.into()
            }
            MacroArg::Block(block_args) => {
                let inner_tokens: proc_macro2::TokenStream = xformat_block(block_args, 1).into();
                quote! { #inner_tokens }.into()
            }
        }
    }

    if args.is_empty() {
        return quote! { "" }.into();
    }

    if args.len() == 1 {
        return xformat_arg(&args[0]);
    }

    let mut iter = args.iter();
    let (format_string, format_color) = match iter.next().unwrap() {
        MacroArg::Simple(expr) => (quote! { #expr }, quote! { broccolor::Color::Transparent }),
        MacroArg::KeyValue(expr, color) => (quote! { #expr }, quote! { #color }),
        MacroArg::Group(inner_args) => {
            let inner_tokens: proc_macro2::TokenStream = xformat_args(inner_args).into();
            (
                quote! { #inner_tokens },
                quote! { broccolor::Color::Transparent },
            )
        }
        MacroArg::Block(block_args) => {
            let inner_tokens: proc_macro2::TokenStream = xformat_block(block_args, 1).into();
            (
                quote! { #inner_tokens },
                quote! { broccolor::Color::Transparent },
            )
        }
    };

    let format_args = iter
        .map(|arg| xformat_arg(arg).into())
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote! {
        format!(
            "{}",
            broccolor::TextStyle::term_colorize(&format!(#format_string, #(#format_args),*), #format_color)
        )
    }.into()
}

fn xformat_block(block: &Vec<MacroArg>, depth: u16) -> proc_macro::TokenStream {
    let depth_lit = depth as usize;
    let mut format_string = String::new();

    let output = block
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            if i != block.len() - 1 {
                format_string.push_str("{}\n");
            } else {
                format_string.push_str("{}");
            }

            match arg {
                MacroArg::Simple(expr) => {
                    quote! {
                        format!("{}{}", "---".repeat(#depth_lit), #expr)
                    }
                }
                MacroArg::KeyValue(expr, color) => {
                    quote! {
                        format!(
                            "{}{}",
                            broccolor::TextStyle::term_colorize(&"---".repeat(#depth_lit), #color),
                            broccolor::ColoredText {
                                content: #expr,
                                color: #color,
                            }
                        )
                    }
                }
                MacroArg::Group(args) => {
                    let inner_args: proc_macro2::TokenStream = xformat_args(args).into();

                    quote! {
                        #inner_args
                    }
                }
                MacroArg::Block(block_args) => {
                    let inner_block = xformat_block(block_args, depth + 1);
                    let inner_block: proc_macro2::TokenStream = inner_block.into();

                    quote! {
                        #inner_block
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        format!(#format_string, #(#output),*)
    }
    .into()
}