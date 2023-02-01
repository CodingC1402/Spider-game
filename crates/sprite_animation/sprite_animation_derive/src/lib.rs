use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(ToUuid)]
pub fn to_uuid_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_to_uuid(&ast)
}

fn impl_to_uuid(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ToUuid for #name {}
    };
    gen.into()
}

#[proc_macro_derive(AnimState)]
pub fn anim_state_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_anim_state(&ast)
}

fn impl_anim_state(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl AnimState for #name {}
    };
    gen.into()
}