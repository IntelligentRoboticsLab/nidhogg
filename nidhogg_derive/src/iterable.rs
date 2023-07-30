use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, GenericParam, Generics, TypeParam};

/// Derive implementation for making structs with a single generic field type iterable.
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident: struct_name,
        data,
        generics,
        ..
    } = parse_macro_input!(input);

    match generics.params.first() {
        Some(GenericParam::Type(TypeParam {
            ident: field_type, ..
        })) => match parse_fields(data) {
            Ok(fields) => {
                let impl_to_vec = impl_to_vec(&struct_name, &generics, &fields);
                let impl_into_iterator =
                    impl_into_iterator(&struct_name, &generics, &fields, field_type);

                quote! {
                    #impl_to_vec
                    #impl_into_iterator
                }
            }
            Err(err) => err,
        },
        other => syn::Error::new_spanned(other, "Only structs with named fields are supported")
            .to_compile_error(),
    }
    .into()
}

fn parse_fields(data: Data) -> Result<Vec<TokenStream>, TokenStream> {
    let fields = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "Only structs with named fields are supported",
                )
                .to_compile_error());
            }
        },
        _ => {
            return Err(syn::Error::new_spanned("", "Only supports structs").to_compile_error());
        }
    };

    if fields.is_empty() {
        panic!("Must contain at least one field!");
    }

    Ok(fields
        .iter()
        .map(|Field { ident, .. }| {
            quote! { self.#ident.clone() }
        })
        .collect())
}

fn impl_to_vec(struct_name: &Ident, generics: &Generics, fields: &Vec<TokenStream>) -> TokenStream {
    let (_, ty_generics, _) = generics.split_for_impl();
    let impl_generics = generic_type_params_with_clone(generics);

    quote! {
        impl <#(#impl_generics)*> #struct_name #ty_generics {
             pub fn to_vec(&self) -> std::vec::Vec #ty_generics {
                 vec![#(#fields), *]
             }
        }
    }
}

fn impl_into_iterator(
    struct_name: &Ident,
    generics: &Generics,
    fields: &Vec<TokenStream>,
    field_type: &Ident,
) -> TokenStream {
    let (_, ty_generics, _) = generics.split_for_impl();
    let impl_generics = generic_type_params_with_clone(generics);

    quote! {
        impl <#(#impl_generics)*> std::iter::IntoIterator for #struct_name #ty_generics {
            type Item = #field_type;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> std::vec::IntoIter<#field_type> {
                vec![#(#fields), *].into_iter()
            }
        }
    }
}

fn generic_type_params_with_clone(generics: &Generics) -> Vec<TokenStream> {
    generics
        .type_params()
        .map(|x| {
            let id = &x.ident;
            let bounds = x.bounds.iter();

            quote! { #id: #(#bounds +)* Clone }
        })
        .collect()
}
