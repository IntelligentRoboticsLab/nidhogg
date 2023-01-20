use itertools::MultiUnzip;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, GenericParam, Generics,
    Ident, Type, TypeParam, Visibility,
};

fn error(loc: &impl syn::spanned::Spanned, msg: &'static str) -> TokenStream {
    syn::Error::new(loc.span(), msg).to_compile_error().into()
}

/// [`Builder`] derive macro implementation
pub fn derive(tokens: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        vis,
        generics,
        data,
        ..
    } = parse_macro_input!(tokens);
    let builder_name = format_ident!("{}Builder", ident);
    let docs = format!("Builder struct for [`{}`]\n", ident.to_string());
    let generic_params = parse_generic_types(generics.clone());

    let builder_type = match generics.gt_token {
        Some(_) => quote! { #builder_name::#generics },
        None => quote! { #builder_name },
    };

    parse_field_data(data).map_or(
        error(&ident, "Builder only supports structs with named fields!"),
        |field_data| {
            let data_name = &field_data.field_names;
            let data_vis = &field_data.field_visibilities;
            let data_type = &field_data.field_types;

            TokenStream::from(quote! {
                #[doc = #docs]
                #[derive(Default)]
                #vis struct #builder_name #generics {
                    #(#data_name: Option<#data_type>),*
                }

                impl <#(#generic_params: Default)*> #builder_name #generics {
                    #(#data_vis fn #data_name (mut self, #data_name: #data_type) -> Self {
                        self.#data_name = Some(#data_name);
                        self
                    })*

                    pub fn build (self) -> #ident #generics {
                        #ident {
                            #(#data_name: self.#data_name.unwrap_or_default()),*
                        }
                    }
                }

                impl <#(#generic_params: Default)*> #ident #generics {
                    pub fn builder() -> #builder_type {
                        #builder_type::default()
                    }
                }
            })
        },
    )
}

struct ParsedFieldData {
    field_names: Vec<Ident>,
    field_visibilities: Vec<Visibility>,
    field_types: Vec<Type>,
}

/// Extract the field names, types and visibilities from a [`Data`] struct.
fn parse_field_data(input: Data) -> Option<ParsedFieldData> {
    let Data::Struct(DataStruct {fields: Fields::Named(FieldsNamed { named, .. }), .. }) = input else { return None };

    let (field_names, field_visibilities, field_types): (Vec<_>, Vec<_>, Vec<_>) = named
        .into_iter()
        .map(|x| (x.ident.unwrap(), x.vis, x.ty))
        .multiunzip();

    Some(ParsedFieldData {
        field_names,
        field_visibilities,
        field_types,
    })
}

/// Extract the generic type parameters from a [`Generics`] struct.
fn parse_generic_types(input: Generics) -> Vec<Ident> {
    input
        .params
        .into_iter()
        .map(|param| match param {
            GenericParam::Type(TypeParam { ident, .. }) => ident,
            _ => unreachable!(),
        })
        .collect()
}
