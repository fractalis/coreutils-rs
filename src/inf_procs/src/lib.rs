use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn main(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let stream = proc_macro2::TokenStream::from(stream);

    let new = quote!(
        pub fn infmain(args: inf_common::InfArgs) -> i32 {
            #stream
            let result = infmain(args);
            println!("Result: {}", result);
            result
        }
    );

    TokenStream::from(new)
}
