//! This crate provides the [`Builder`] macro used in nidhogg.
use proc_macro::TokenStream;

mod builder;
mod iterable;

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

/// Derive macro to implement tools for iterating over the fields of a struct that has fields of
/// the same types. Implements trait `IntoIter` and function `to_vec`.
///
/// These fields in this struct need to implement [`Clone`]
///
/// ## Examples
/// ```no_run
/// use nidhogg_derive::Iterable;
///
/// #[derive(Iterable)]
/// struct Foo<T> {
///     bar: T,
///     baz: T,
/// }
///
/// let foo = Foo { bar: 5, baz: 21 };
/// let mut foo_iterator = foo.into_iter();
/// assert_eq!(foo_iterator.next().unwrap(), 5);
/// assert_eq!(foo_iterator.next().unwrap(), 21);
/// ```
#[proc_macro_derive(Iterable)]
pub fn derive_iterable(input: TokenStream) -> TokenStream {
    iterable::derive(input)
}
