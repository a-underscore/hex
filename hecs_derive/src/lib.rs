use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component, attributes(component))]
pub fn component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics AsAny for #name #ty_generics #where_clause {
            fn as_any(self: Rc<Self>) -> Rc<dyn Any + 'static> {
                self.clone()
            }
        }

        impl #impl_generics PartialEq for #name #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                *self.tid() == *other.tid() && *self.id() == *other.id()
            }
        }
    };

    TokenStream::from(expanded)
}
