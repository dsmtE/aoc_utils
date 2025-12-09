use proc_macro::TokenStream;
use quote::quote;
use syn::{Token, ItemFn, ReturnType, parse_macro_input};

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn todo_to_optional_result(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut func: ItemFn = parse_macro_input!(item as ItemFn);

    // detect if todo!() macro is used in the function body
    // This is a very naive implementation that only checks for
    // direct usage of todo!() in the top-level statements of the function.
    // TODO: Improve it to handle nested blocks, expressions, etc.
    let todo_used = func.block.stmts.iter().any(|stmt| {
        if let syn::Stmt::Expr(expr,_) = stmt {
            if let syn::Expr::Macro(mac) = expr {
                if mac.mac.path.is_ident("todo") {
                    return true;
                }
            }
        }
        return false;
    });

    func.sig.output = match func.sig.output {
        ReturnType::Default => ReturnType::Type(
            Token![->](proc_macro2::Span::call_site()),
            Box::new(syn::parse_quote! { Option<()> }),
        ),
        ReturnType::Type(arrow, ty) => ReturnType::Type(
            arrow,
            Box::new(syn::parse_quote! { Option<#ty> }),
        ),
    };

    let block = &mut func.block;
    if todo_used {
        *block = syn::parse_quote! { {None} };
    } else {
        // should move inside a closure ? : Some((|| #block)())
        *block = syn::parse_quote! { {Some( #block )} };
    }

    quote! { #func }.into()
}