use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, GenericParam, Generics, TypeParam};

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

    //field_type
    match generics.params.first() {
        Some(GenericParam::Type(TypeParam {
            ident: field_type, ..
        })) => {
            let fields = parse_fields(data);
            impl_into_iterator(&struct_name, &generics, &fields, field_type).into()
        }
        _ => panic!("Must use exactly one generic!"),
    }
}

fn parse_fields(data: Data) -> Vec<TokenStream> {
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

    // Should always be possible, since a struct with a generic will have fields.
    let field_type = &fields[0].ty;

    fields
        .iter()
        .map(|Field { ident, ty, .. }| {
            if ty != field_type {
                panic!("All fields must be of the same type");
            }
            quote! { self.#ident.clone() }
        })
        .collect()
}

fn impl_into_iterator(
    struct_name: &Ident,
    generics: &Generics,
    fields_iter: &Vec<TokenStream>,
    field_type: &Ident,
) -> TokenStream {
    let (_, ty_generics, _) = generics.split_for_impl();
    let impl_generics_test = generic_type_params_with_clone(generics);

    quote! {
        impl <#(#impl_generics_test)*> FillExt<#field_type> for #struct_name #ty_generics {
            fn fill(value: #field_type) -> Self {
                Self {
                    #(#fields_iter: value), *
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
