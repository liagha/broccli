#![allow(dead_code)]

mod parse;
use parse::{MacroArg, MacroArgs};
use proc_macro::TokenStream;
use quote::{quote};
use syn::{Expr, ExprBlock, parse_macro_input, Token, Stmt};

#[proc_macro]
pub fn xformat_args(input: TokenStream) -> TokenStream {
    let MacroArgs { mut args } = parse_macro_input!(input as MacroArgs);

    if args.is_empty() {
        return quote! {
            println!("")
        }
        .into();
    }

    let (format, format_color) = match args.remove(0) {
        MacroArg::Simple(arg) => {
            let color: Expr = syn::parse_quote! { Color::White };

            (arg, color)
        }
        MacroArg::KeyValue(arg, color) => (arg, color),
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

fn xformat_args_fn(input: TokenStream) -> TokenStream {
    println!("{}", input);

    let MacroArgs { mut args } = parse_macro_input!(input as MacroArgs);

    if args.is_empty() {
        return quote! {
            println!("")
        }
        .into();
    }

    let (format, format_color) = match args.remove(0) {
        MacroArg::Simple(arg) => {
            let color: Expr = syn::parse_quote! { broccolor::Color::Transparent };

            (arg, color)
        }
        MacroArg::KeyValue(arg, color) => (arg, color),
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

use syn::punctuated::Punctuated;

#[proc_macro]
pub fn tree(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let exprs = parse_macro_input!(input with parser);
    process_exprs(&exprs, Vec::new()).into()
}

fn process_exprs(exprs: &Punctuated<Expr, Token![,]>, prefix_stack: Vec<bool>) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    for (i, expr) in exprs.iter().enumerate() {
        let last = i == exprs.len() - 1;

        let mut prefix = generate_prefix(&prefix_stack);
        prefix.push_str(if last { "└──" } else { "├──" });

        match expr {
            Expr::Block(ExprBlock { block, .. }) => {
                tokens.extend(quote! { println!("{}─┐", #prefix); });

                let mut new_prefix_stack = prefix_stack.clone();
                new_prefix_stack.push(!last);

                for (j, stmt) in block.stmts.iter().enumerate() {
                    let stmt_last = j == block.stmts.len() - 1;
                    tokens.extend(process_stmt(stmt, new_prefix_stack.clone(), stmt_last));
                }
            }
            _ => tokens.extend(quote! { println!("{} {}", #prefix, stringify!(#expr)); }),
        }
    }

    tokens
}

fn process_stmt(stmt: &Stmt, prefix_stack: Vec<bool>, is_last: bool) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();

    let mut prefix = generate_prefix(&prefix_stack);
    prefix.push_str(if is_last { "└──" } else { "├──" });

    match stmt {
        Stmt::Expr(nested_expr, _) => {
            if let Expr::Block(ExprBlock { block, .. }) = nested_expr {
                tokens.extend(quote! { println!("{}─┐", #prefix); });

                let mut new_prefix_stack = prefix_stack.clone();
                new_prefix_stack.push(!is_last);

                for (j, stmt) in block.stmts.iter().enumerate() {
                    let stmt_last = j == block.stmts.len() - 1;
                    tokens.extend(process_stmt(stmt, new_prefix_stack.clone(), stmt_last));
                }
            } else {
                let formatted_nested_expr : proc_macro2::TokenStream = xformat_args_fn(quote! { #nested_expr }.into()).into();
                tokens.extend(quote! {
                    let text = #formatted_nested_expr;
                    println!("{} {}", #prefix, text);
                });
            }
        }
        _ => {
            let formatted_stmt : proc_macro2::TokenStream = xformat_args_fn(quote! { #stmt }.into()).into();
            tokens.extend(quote! {
                let text = #formatted_stmt;
                println!("{} {}", #prefix, text)
            });
        },
    }

    tokens
}

fn generate_prefix(prefix_stack: &[bool]) -> String {
    let mut prefix = String::new();
    for &has_pipe in prefix_stack {
        prefix.push_str(if has_pipe { "│   " } else { "    " });
    }
    prefix
}
