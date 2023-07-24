use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Generics};

/// Trait that introduces the [`fill`](`FillExt::fill`) method for a type, which allows filling in all fields with the same value.
pub trait FillExt<T> {
    /// Return a new instance of the type, with all fields set to the provided value.
    fn fill(value: T) -> Self;
}

/// Derive implementation for function that fills struct with one fixed value.
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input);
    let struct_name = ident;

    match generics.params.first() {
        Some(_) => {
            let (fields, field_type) = parse_fields(data);
            let (_, ty_generics, _) = generics.split_for_impl();
            let impl_generics_test = generic_type_params_with_clone(&generics);

            quote! {
                impl<#(#impl_generics_test)*> FillExt<#field_type> for #struct_name #ty_generics {
                    fn fill(value: #field_type) -> #struct_name<#field_type> {
                        #struct_name {
                            #( #fields: value.clone() ), *
                        }
                    }
                }
            }
        }
        None => {
            let (fields, field_type) = parse_fields(data);
            quote! {
                impl FillExt<#field_type> for #struct_name {
                    fn fill(value: #field_type) -> #struct_name {
                        #struct_name {
                            #( #fields: value ), *
                        }
                    }
                }
            }
        }
    }
    .into()
}

fn parse_fields(data: Data) -> (Vec<TokenStream>, syn::Type) {
    let fields = match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only supports structs"),
    };

    if fields.is_empty() {
        panic!("Must contain at least one field!");
    }

    let field_type = &fields[0].ty;

    (
        fields
            .iter()
            .map(|Field { ident, ty, .. }| {
                if ty != field_type {
                    panic!("All fields must be of the same type");
                }
                quote! { #ident }
            })
            .collect(),
        field_type.clone(),
    )
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
