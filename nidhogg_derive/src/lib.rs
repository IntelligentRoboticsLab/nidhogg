use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, Ident, Type};

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let field_data = parse_field_data(input.data).expect("Struct doesn't contain any named fields");
    let data_name = &field_data.field_names;
    let data_type = &field_data.field_types;

    let expanded = quote! {
        #[derive(Default)]
        struct #builder_name {
            #(#data_name: Option<#data_type>),*
        }

        impl #builder_name {
            #(pub fn #data_name (mut self, #data_name: #data_type) -> Self {
                self.#data_name = Some(#data_name);
                self
            })*

            pub fn build (self) -> #name {
                #name {
                    #(#data_name: self.#data_name.unwrap_or_default()),*
                }
            }
        }

        impl #name {
            fn builder() -> #builder_name {
                #builder_name::default()
            }
        }
    };

    TokenStream::from(expanded)
}

struct ParsedFieldData {
    field_names: Vec<Ident>,
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
        field_types: field_data
            .named
            .iter()
            .cloned()
            .map(|field| field.ty)
            .collect(),
    })
}
