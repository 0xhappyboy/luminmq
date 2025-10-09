use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Ident, ItemFn, Lit, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

const REQUIRED_ATTR_KEY_1: &str = "group_id";
const REQUIRED_ATTR_KEY_2: &str = "topic";

struct ConsumerAttrs {
    group_id: LitStr,
    topic: LitStr,
}

impl Parse for ConsumerAttrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group_id_ident: Ident = input.parse()?;
        if group_id_ident != REQUIRED_ATTR_KEY_1 {
            return Err(syn::Error::new(
                group_id_ident.span(),
                "Expected 'group_id'",
            ));
        }
        input.parse::<Token![=]>()?;
        let group_id_lit: Lit = input.parse()?;
        let group_id = match group_id_lit {
            Lit::Str(lit_str) => lit_str,
            _ => {
                return Err(syn::Error::new(
                    group_id_lit.span(),
                    "Expected string literal for 'group_id'",
                ));
            }
        };
        input.parse::<Token![,]>()?;
        let topic_ident: Ident = input.parse()?;
        if topic_ident != REQUIRED_ATTR_KEY_2 {
            return Err(syn::Error::new(topic_ident.span(), "Expected 'topic'"));
        }
        input.parse::<Token![=]>()?;
        let topic_lit: Lit = input.parse()?;
        let topic = match topic_lit {
            Lit::Str(lit_str) => lit_str,
            _ => {
                return Err(syn::Error::new(
                    topic_lit.span(),
                    "Expected string literal for 'topic'",
                ));
            }
        };
        Ok(ConsumerAttrs { group_id, topic })
    }
}

/// a consumer function used to mark a message queue.
/// The function signature must be in the following format:
/// ```rust
/// pub fn consumer(message: Message) -> Result<String, String>
/// ```
/// Example
/// ```rust
/// #[consumer(group_id = "group-test", topic = "topic-test")]
/// fn say_bye(message: Message) -> Result<String, String> {
///    // ......
///  }
/// ```
#[proc_macro_attribute]
pub fn consumer(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ConsumerAttrs);
    let group_id_str = args.group_id.value();
    let topic_str = args.topic.value();
    if group_id_str.is_empty() {
        panic!("group id cannot be empty")
    }
    if topic_str.is_empty() {
        panic!("topic cannot be empty")
    }
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_ident = &input_fn.sig.ident;
    let fn_name_str = fn_ident.to_string();
    let register_ident = Ident::new(&format!("register_{}", fn_name_str), Span::call_site());
    let output = quote! {
        #input_fn
        #[ctor::ctor]
        #[allow(non_snake_case)]
        fn #register_ident() {
             luminmq_core::types::ConsumerBinder::insert((#group_id_str.to_string(),#topic_str.to_string()),#fn_ident);
        }
    };
    output.into()
}
