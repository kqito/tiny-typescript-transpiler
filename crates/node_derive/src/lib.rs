use proc_macro::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, parse_macro_input, Data, DeriveInput, Error};

#[proc_macro_derive(Node)]
pub fn node_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    match generate_node(input) {
        Ok(generated) => generated,
        Err(err) => err.to_compile_error().into(),
    }
}

fn generate_node(derive_input: &DeriveInput) -> Result<TokenStream, Error> {
    let struct_data = match &derive_input.data {
        Data::Struct(v) => v,
        _ => {
            return Err(Error::new_spanned(
                &derive_input.ident,
                "Mut be struct type",
            ))
        }
    };

    let mut quotes = Vec::new();

    for field in &struct_data.fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let method_name: proc_macro2::TokenStream = format!("get_{}", ident.unraw().to_string())
            .parse()
            .unwrap();

        quotes.push(quote! {
            pub fn #method_name(&self) -> #ty {
                self.#ident.clone()
            }
        });
    }

    let struct_name = &derive_input.ident;
    let (impl_generics, _, where_clause) = &derive_input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #struct_name #where_clause {
            #(#quotes)*
        }
    };

    Ok(expanded.into())
}
