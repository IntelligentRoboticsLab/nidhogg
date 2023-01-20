use proc_macro::TokenStream;

mod builder;

/// Derive macro  to implement the [builder pattern](https://refactoring.guru/design-patterns/builder)
/// for an arbritary struct with named fields.
/// 
/// These fields in this struct need to implement [`Default`]
/// 
/// ## Examples
/// ```no_run
/// use nidhogg_derive::Builder;
/// 
/// #[derive(Builder, Default, Eq)]
/// struct Foo {
///     bar: i32,
///     baz: Vec<u8>
/// }
/// 
/// fn main () {
///     let foo = Foo::builder().bar(42).baz(vec![4, 2]).build();
///     assert_eq!(foo, Foo { bar: 42, baz: vec![4, 2]})
/// }
/// ```
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
   builder::derive(input)
}
