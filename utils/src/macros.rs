#[macro_export]
macro_rules! simple_test {
    ($test_name:ident, $the_fn:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let actual = $the_fn(& $input.to_string());
            assert_eq!(
                format!("{}", $expected ),
                format!("{}", actual),
            );
        }
    };
}

#[macro_export]
macro_rules! simple_test2 {
    ($test_name:ident, $the_fn:ident, $arg:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let actual = $the_fn(& ($arg.0).to_string(), $arg.1);
            assert_eq!(
                format!("{}", $expected ),
                format!("{}", actual),
            );
        }
    };
}
