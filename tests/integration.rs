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
}