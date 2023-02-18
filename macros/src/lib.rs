use proc_macro::TokenStream;
use quote::quote;



#[proc_macro_attribute]
pub fn get(_args: TokenStream, _input: TokenStream) -> TokenStream {
    
    format!("fn oobilydoop() {{ println!({}); }}", _input.to_string()).parse().unwrap()
}