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
        })) => match parse_data(data) {
            Ok((fields_clone, fields)) => {
                let impl_to_vec = impl_to_vec(&struct_name, &generics, &fields_clone);
                let impl_into_iterator =
                    impl_into_iterator(&struct_name, &generics, &fields_clone, field_type);
                let impl_from_iterator =
                    impl_from_iterator(&struct_name, &generics, &fields, field_type);

                quote! {
                    #impl_to_vec

                    #impl_into_iterator

                    #impl_from_iterator
                }
            }
            Err(err) => err,
        },
        other => syn::Error::new_spanned(other, "Only structs with named fields are supported")
            .to_compile_error(),
    }
    .into()
}

fn parse_data(data: Data) -> Result<(Vec<TokenStream>, Vec<TokenStream>), TokenStream> {
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
        return Err(
            syn::Error::new_spanned(fields, "Must contain at least one field!").to_compile_error(),
        );
    }

    Ok((
        fields
            .iter()
            .map(|Field { ident, .. }| {
                quote! { self.#ident.clone() }
            })
            .collect(),
        fields
            .iter()
            .map(|Field { ident, .. }| {
                quote! { #ident }
            })
            .collect(),
    ))
}

fn impl_to_vec(
    struct_name: &Ident,
    generics: &Generics,
    self_fields_with_clone: &Vec<TokenStream>,
) -> TokenStream {
    let (_, ty_generics, _) = generics.split_for_impl();
    let impl_generics = generic_type_params_with_clone(generics);

    quote! {
        impl <#(#impl_generics)*> #struct_name #ty_generics {
             pub fn to_vec(&self) -> std::vec::Vec #ty_generics {
                 vec![#(#self_fields_with_clone), *]
             }
        }
    }
}

fn impl_into_iterator(
    struct_name: &Ident,
    generics: &Generics,
    self_fields_with_clone: &Vec<TokenStream>,
    field_type: &Ident,
) -> TokenStream {
    let (_, ty_generics, _) = generics.split_for_impl();
    let impl_generics = generic_type_params_with_clone(generics);

    quote! {
        impl <#(#impl_generics)*> std::iter::IntoIterator for #struct_name #ty_generics {
            type Item = #field_type;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> std::vec::IntoIter<#field_type> {
                vec![#(#self_fields_with_clone), *].into_iter()
            }
        }
    }
}

fn impl_from_iterator(
    struct_name: &Ident,
    generics: &Generics,
    fields: &Vec<TokenStream>,
    field_type: &Ident,
) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let number_of_fields = fields.len();

    quote! {
        impl #impl_generics FromIterator<#field_type> for #struct_name #ty_generics {
            fn from_iter<I: IntoIterator<Item=#field_type>>(iter: I) -> Self {
                let mut collector: Vec<#field_type> = Vec::new();

                for item in iter {
                    collector.push(item);
                }

                let collector_len = collector.len();
                let Ok([#(#fields), *]): Result<[#field_type; #number_of_fields], Vec<#field_type>> = collector.try_into() else {
                    panic!("Not the correct number of values in iterator expected {:?}, values got {:?}.", #number_of_fields, collector_len);
                };
                Self {
                    #(#fields), *
                }
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
