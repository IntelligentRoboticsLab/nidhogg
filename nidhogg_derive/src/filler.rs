use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Generics, Ident, Type};

/// Derive implementation for function that fills struct with one fixed value.
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input);
    match parse_fields(data, &ident) {
        Ok((fields, field_type)) => gen_filler_impl(&generics, &ident, &fields, &field_type),
        Err(err) => err,
    }
    .into()
}

fn gen_filler_impl(
    generics: &Generics,
    struct_name: &Ident,
    fields: &Vec<TokenStream>,
    field_type: &Type,
) -> TokenStream {
    match generics.params.first() {
        Some(_) => {
            let (_, ty_generics, where_clause) = generics.split_for_impl();
            let impl_generics_test = generic_type_params_with_clone(generics);

            quote! {
                impl<#(#impl_generics_test)*> crate::types::FillExt<#field_type> for #struct_name #ty_generics #where_clause {
                    fn fill(value: #field_type) -> Self {
                        #struct_name {
                            #( #fields: value.clone() ), *
                        }
                    }
                }
            }
        }
        None => {
            quote! {
                impl crate::types::FillExt<#field_type> for #struct_name {
                    fn fill(value: #field_type) -> Self {
                        #struct_name {
                            #( #fields: value.clone() ), *
                        }
                    }
                }
            }
        }
    }
}

fn parse_fields(
    data: Data,
    struct_name: &Ident,
) -> Result<(Vec<TokenStream>, syn::Type), TokenStream> {
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
            return Err(
                syn::Error::new_spanned(struct_name, "Only supports structs").to_compile_error(),
            );
        }
    };

    if fields.is_empty() {
        return Err(
            syn::Error::new_spanned(struct_name, "Only supports structs").to_compile_error(),
        );
    }

    let field_type = &fields[0].ty;

    Ok((
        fields
            .iter()
            .map(|Field { ident, .. }| {
                quote! { #ident }
            })
            .collect(),
        field_type.clone(),
    ))
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
