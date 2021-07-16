#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn failing_test() {
        panic!("This test has failed");
    }

    #[test]
    fn eq_macro() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    fn ne_macro() {
        assert_ne!(3, 1 + 1);
    }

    #[test]
    // notice that we get a reason for test failure!
    fn eq_macro_info() {
        assert_eq!(0, 2);
    }
}
