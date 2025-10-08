use std::collections::HashMap;

use luminmq_core::types::ConsumerBinder;
use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, parse_macro_input};

const REQUIRED_TYPE: &str = "Message";
const REQUIRED_ATTR_KEY_1: &str = "group_id";
const REQUIRED_ATTR_KEY_2: &str = "topic";

#[proc_macro_attribute]
pub fn consumer(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let mut flag: bool = false;
    // attribute list
    let mut cache = HashMap::<String, String>::new();
    let attrs_str = attr.to_string();
    if !attrs_str.is_empty() {
        let attrs: Vec<&str> = attrs_str.split(',').collect();
        attrs.iter().for_each(|att| {
            let v: Vec<&str> = att.split("=").collect();
            if v.len() == 2 {
                let k = v.get(0).unwrap().trim();
                let v = v.get(1).unwrap().trim();
                cache.insert(k.to_string(), v.to_string());
            }
        });
    }
    if cache.contains_key(REQUIRED_ATTR_KEY_1) && cache.contains_key(REQUIRED_ATTR_KEY_2) {
        flag = true;
    }
    // list of function parameter types.
    let mut arg_types = vec![];
    for arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            let ty = &pat_type.ty;
            arg_types.push(ty);
        }
    }
    let types: Vec<String> = arg_types.iter().map(|ty| quote!(#ty).to_string()).collect();
    if types.len() == 1 {
        types.iter().for_each(|t| {
            if REQUIRED_TYPE.eq(t) {
                flag = true;
            } else {
                flag = false;
            }
        });
    }
    if flag {
        ConsumerBinder::insert(
            (
                cache.get(REQUIRED_ATTR_KEY_1).unwrap().to_string(),
                cache.get(REQUIRED_ATTR_KEY_2).unwrap().to_string(),
            ),
            fn_name.to_string(),
        );
    }
    let output = quote! {
        #input_fn
    };
    output.into()
}

#[proc_macro]
pub fn exec_consume(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ExprTuple);
    if input.elems.len() > 2 {
        panic!("Expected a tuple with three elements: (function_name, arg1)");
    }
    let fn_name = input.elems[0].clone();
    let arg1 = input.elems[1].clone();
    let fn_name = if let syn::Expr::Lit(expr_lit) = &fn_name {
        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
            lit_str
        } else {
            panic!("Function name must be a string literal");
        }
    } else {
        panic!("Function name must be a string literal");
    };
    let func_ident = syn::Ident::new(&fn_name.value(), fn_name.span());
    let expanded = quote! {
        #func_ident(#arg1)
    };
    TokenStream::from(expanded)
}
