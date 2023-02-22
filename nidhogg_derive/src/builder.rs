use itertools::MultiUnzip;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Generics, Ident, Type,
    Visibility,
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
    let data_name = field_data.field_names.as_slice();
    let data_vis = field_data.field_visibilities.as_slice();
    let data_type = field_data.field_types.as_slice();

    let generics_no_type_bounds = generic_types(generics);
    let generic_type_params = generic_type_params_with_default(generics);

    quote!(
        impl <#(#generic_type_params)*> #builder_name <#(#generics_no_type_bounds),*> {
            #(#data_vis fn #data_name (mut self, #data_name: #data_type) -> Self {
                self.#data_name = Some(#data_name);
                self
            })*

            pub fn build (self) -> #ident<#(#generics_no_type_bounds),*> {
                #ident {
                    #(#data_name: self.#data_name.unwrap_or_default()),*
                }
            }
        }
    )
}

fn impl_builder_fn(ident: &Ident, builder_name: &Ident, generics: &Generics) -> TokenStream {
    let generic_type_params = generic_type_params_with_default(generics);
    let generics_no_type_bounds = generic_types(generics);

    let builder_type = match generics.gt_token {
        Some(_) => quote! { #builder_name::<#(#generics_no_type_bounds),*> },
        None => quote! { #builder_name },
    };

    quote! {
        impl <#(#generic_type_params)*> #ident <#(#generics_no_type_bounds),*> {
            pub fn builder() -> #builder_type {
                #builder_type::default()
            }
        }
    }
}

fn generic_types(generics: &Generics) -> Vec<Ident> {
    generics.type_params().map(|x| x.ident.to_owned()).collect()
}

fn generic_type_params_with_default(generics: &Generics) -> Vec<TokenStream> {
    generics
        .type_params()
        .map(|x| {
            let id = &x.ident;
            let bounds = x.bounds.iter();

            if bounds.len() > 0 {
                quote! { #id: #(#bounds)+* + Default }
            } else {
                quote! { #id: Default }
            }
        })
        .collect()
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
