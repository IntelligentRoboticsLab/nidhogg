#[cfg(test)]
mod iterable_tests {
    use nidhogg_derive::Iterable;

    /// Deriving `Iterable` on the following struct kinds should result in a compile-error:
    /// ```compile_fail
    /// use nidhogg_derive::Iterable;
    /// #[derive(Iterable)]
    /// struct DoesNotCompile(i32);
    ///```
    #[derive(Debug, Iterable)]
    pub struct TestStruct<T> {
        pub test_field: T,
    }

    #[test]
    fn it_works() {
        let t = TestStruct { test_field: 5 };

        for elem in t.into_iter() {
            assert_eq!(elem, 5)
        }
    }
}
