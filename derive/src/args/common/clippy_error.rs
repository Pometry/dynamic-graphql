use proc_macro2::TokenStream;
use quote::quote;
use syn::Generics;

pub fn impl_suppress_tupple_clippy_error(
    ident: &syn::Ident,
    generics: &Generics,
    len: usize,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let accesses: TokenStream = (0..len)
        .map(|index| {
            let index = syn::Index::from(index);
            quote! {
                let _ = self.#index;
            }
        })
        .collect();
    quote! {
         impl #impl_generics #ident #ty_generics #where_clause {
             #[allow(dead_code)]
             #[doc(hidden)]
             fn __suppress_clippy_error(&self) {
                 #accesses
             }
         }
    }
}
