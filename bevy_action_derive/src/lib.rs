use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Action)]
pub fn bevy_action_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        impl Action for #name {}
    };

    gen.into()
}
