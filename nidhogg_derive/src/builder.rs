use itertools::MultiUnzip;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, GenericParam, Generics,
    Ident, Type, TypeParam, Visibility,
};

fn error(loc: &impl syn::spanned::Spanned, msg: &'static str) -> proc_macro::TokenStream {
    syn::Error::new(loc.span(), msg).to_compile_error().into()
}

/// [`Builder`] derive macro implementation
pub fn derive(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        vis,
        generics,
        data,
        ..
    } = parse_macro_input!(tokens);
    let builder_name = format_ident!("{}Builder", ident);

    parse_field_data(data).map_or(
        error(&ident, "Builder only supports structs with named fields!"),
        |field_data| {
            let builder_struct =
                builder_struct(&ident, &builder_name, &vis, &generics, &field_data);
            let impl_builder_struct =
                impl_builder_struct(&ident, &builder_name, &field_data, &generics);
            let impl_builder_fn = impl_builder_fn(&ident, &builder_name, &generics);

            quote! {
                 #builder_struct

                 #impl_builder_struct

                 #impl_builder_fn
            }
            .into()
        },
    )
}

fn builder_struct(
    ident: &Ident,
    builder_name: &Ident,
    vis: &Visibility,
    generics: &Generics,
    field_data: &ParsedFieldData,
) -> TokenStream {
    let docs = format!("Builder struct for [`{}`]\n", ident);
    let data_name = &field_data.field_names;
    let data_type = &field_data.field_types;
    quote!(
        #[doc = #docs]
        #[derive(Default)]
        #vis struct #builder_name #generics {
            #(#data_name: Option<#data_type>),*
        }
    )
}

fn impl_builder_struct(
    ident: &Ident,
    builder_name: &Ident,
    field_data: &ParsedFieldData,
    generics: &Generics,
) -> TokenStream {
    let generic_params = parse_generic_types(generics);
    let data_name = field_data.field_names.as_slice();
    let data_vis = field_data.field_visibilities.as_slice();
    let data_type = field_data.field_types.as_slice();

    quote!(
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
    )
}

fn impl_builder_fn(ident: &Ident, builder_name: &Ident, generics: &Generics) -> TokenStream {
    let generic_params = parse_generic_types(generics);

    let builder_type = match generics.gt_token {
        Some(_) => quote! { #builder_name::#generics },
        None => quote! { #builder_name },
    };

    quote! {
        impl <#(#generic_params: Default)*> #ident #generics {
            pub fn builder() -> #builder_type {
                #builder_type::default()
            }
        }
    }
}

struct ParsedFieldData {
    field_names: Vec<Ident>,
    field_visibilities: Vec<Visibility>,
    field_types: Vec<Type>,
}

/// Extract the field names, types and visibilities from a [`Data`] struct.
fn parse_field_data(input: Data) -> Option<ParsedFieldData> {
    let Data::Struct(DataStruct {fields: Fields::Named(FieldsNamed { named, .. }), .. }) = input else { return None };

    let (field_names, field_visibilities, field_types) = named
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
fn parse_generic_types(input: &Generics) -> Vec<&Ident> {
    input
        .params
        .iter()
        .flat_map(|p| match p {
            GenericParam::Type(TypeParam { ident, .. }) => Some(ident),
            _ => None,
        })
        .collect()
}
