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
        pub test_field_1: T,
        pub test_field_2: T,
    }

    #[test]
    fn into_iter_works() {
        let t = TestStruct {
            test_field_1: 5,
            test_field_2: -5,
        };

        for (a, b) in t.into_iter().zip(vec![5, -5]) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn from_iter_works() {
        let t0 = TestStruct {
            test_field_1: 5,
            test_field_2: -5,
        };

        let t1 = TestStruct {
            test_field_1: 11,
            test_field_2: -3,
        };

        let t2: TestStruct<i32> = t0
            .into_iter()
            .zip(t1.into_iter())
            .map(|(a, b)| (a + b) / 2)
            .collect();

        for (a, b) in t2.into_iter().zip(vec![8, -4]) {
            assert_eq!(a, b);
        }
    }

    #[test]
    #[should_panic]
    fn from_iter() {
        let _ = TestStruct::from_iter(vec![1, 2, 3]);
    }
}
