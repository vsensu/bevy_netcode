use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct or enum
    let name = &input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl MyTrait for #name {
            fn my_function(&self) {
                println!("This is a custom implementation for {}", stringify!(#name));
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}
