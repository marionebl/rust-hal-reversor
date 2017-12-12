extern crate assert_cli;

#[cfg(test)]
mod integration {
    use assert_cli;

    #[test]
    fn without_args() {
        assert_cli::Assert::main_binary()
            .fails()
            .and()
            .stderr().contains("requires 1 arguments, received 0")
            .unwrap();
    }

    #[test]
    fn with_too_many_args() {
        assert_cli::Assert::main_binary()
            .with_args(&["foo", "bar"])
            .fails()
            .and()
            .stderr().contains("requires 1 arguments, received 2")
            .unwrap();
    }


    #[test]
    fn with_args() {
        assert_cli::Assert::main_binary()
            .with_args(&["foo"])
            .stdout().contains("oof")
            .unwrap();
    }
}