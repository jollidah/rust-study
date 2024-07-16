#[cfg(test)]
pub mod test {
    use std::fmt::Debug;

    pub trait AssertForTest {
        fn compare_input_expected(&self);
    }

    impl<T: PartialEq + Debug> AssertForTest for TestCase<T> {
        fn compare_input_expected(&self) {
            let (input, expected) = (&self.input, &self.expected);
            assert_eq!(
                *input,
                *expected,
                "{}",
                &format!("{:?} is not same with {:?}", input, expected)
            );
        }
    }
    pub struct TestCase<T> {
        pub(crate) input: T,
        pub(crate) expected: T,
    }
}
