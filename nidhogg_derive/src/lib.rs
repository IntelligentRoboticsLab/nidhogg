//! This crate provides the [`Builder`] macro used in nidhogg.
use proc_macro::TokenStream;

mod builder;
mod filler;

/// Derive macro to implement the [builder pattern](https://refactoring.guru/design-patterns/builder)
/// for an arbitrary struct with named fields.
///
/// These fields in this struct need to implement [`Default`]
///
/// ## Examples
/// ```no_run
/// use nidhogg_derive::Builder;
///
/// #[derive(Builder, Debug, Default, PartialEq)]
/// struct Foo {
///     bar: i32,
///     baz: Vec<u8>
/// }
///
/// let foo = Foo::builder().bar(42).baz(vec![4, 2]).build();
/// assert_eq!(foo, Foo { bar: 42, baz: vec![4, 2]})
/// ```
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    builder::derive(input)
}

#[proc_macro_derive(Filler)]
pub fn derive_filler(input: TokenStream) -> TokenStream {
    filler::derive(input)
}
