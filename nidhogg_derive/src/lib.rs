use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, GenericParam, Generics, Ident, Type,
    TypeParam, Visibility,
};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);
    let generics = input.generics;
    let vis = input.vis;

    let field_data = parse_field_data(input.data).expect("Struct doesn't contain any named fields");
    let data_name = &field_data.field_names;
    let data_vis = &field_data.field_visibilities;
    let data_type = &field_data.field_types;

    let builder_type = if generics.gt_token.is_some() {
        quote! {
            #builder_name::#generics
        }
    } else {
        quote! {
            #builder_name
        }
    };

    let generic_params = &parse_generic_types(&generics);

    let expanded = quote! {
        #[derive(Default)]
        #vis struct #builder_name #generics {
            #(#data_name: Option<#data_type>),*
        }

        impl <#(#generic_params: Default)*> #builder_name #generics {
            #(#data_vis fn #data_name (mut self, #data_name: #data_type) -> Self {
                self.#data_name = Some(#data_name);
                self
            })*

            pub fn build (self) -> #name #generics {
                #name {
                    #(#data_name: self.#data_name.unwrap_or_default()),*
                }
            }
        }

        impl <#(#generic_params: Default)*> #name #generics {
            pub fn builder() -> #builder_type {
                #builder_type::default()
            }
        }
    };

    TokenStream::from(expanded)
}

struct ParsedFieldData {
    field_names: Vec<Ident>,
    field_visibilities: Vec<Visibility>,
    field_types: Vec<Type>,
}

fn parse_field_data(input: Data) -> Option<ParsedFieldData> {
    let data: Option<FieldsNamed> = match input {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named) => Some(named),
            _ => None,
        },
        _ => None,
    };

    data.map(|field_data| ParsedFieldData {
        field_names: field_data
            .named
            .iter()
            .cloned()
            .map(|field| field.ident.unwrap())
            .collect(),
        field_visibilities: field_data
            .named
            .iter()
            .cloned()
            .map(|field| field.vis)
            .collect(),
        field_types: field_data
            .named
            .iter()
            .cloned()
            .map(|field| field.ty)
            .collect(),
    })
}

fn parse_generic_types(input: &Generics) -> Vec<Ident> {
    input
        .params
        .iter()
        .cloned()
        .map(|p| match p {
            GenericParam::Type(t) => t,
            _ => unreachable!(),
        })
        .map(|t| t.ident)
        .collect()
}
