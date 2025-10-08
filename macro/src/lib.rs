use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

/// Used to mark message consumption functions.
#[proc_macro_attribute]
pub fn consumer(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    // arg 0 : group id
    // arg 1 : topic
    for arg in attr_args {
        // ..
    }

    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;

    let output = quote! {
        #input
        pub fn #fn_name() {
            #fn_body
        }
    };

    TokenStream::from(output)
}
