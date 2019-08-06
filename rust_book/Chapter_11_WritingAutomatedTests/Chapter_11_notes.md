# Automated Testing
* Type checking != intent checking
* i.e.- a piece of code can have proper syntax/type declaration, etc. but contain bugs in relation to the expected result/output
* Testing in Rust (like `testhat` in R) works by assertion/expectation 

## Writing Tests in Rust
* essentially a function that's annotated with the `test` attribute
```
# [cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
}
```
* the `#[test]` annotation indicates that the next function is a test
* `assert_eq` is a macro that does exactly what it sounds like
* Run tests with
```
cargo test
```
* each test is run on a new thread and its status is recorded by the main thread
    * this means that all tests can run even if one fails
```
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```
* general tests with `assert!` + any function that returns a Boolean
* the argument order is irrelevant, simply tests `left_arg` vs `right_arg`
* scoped versions with
    * `assert_eq!` -> equality
    * `assert_ne!` -> inequality
    ```
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }
    ```
* `#[cfg(test)]` tells Rust to compile/run subsequent code only when using `cargo test`
* can use `./tests/` directory with multiple test files
* setup scripts for tests can go in `./tests/common/mod.rs`
