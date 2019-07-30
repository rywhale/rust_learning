# Error Handling
* Rust often requires you account for the possibility of an error before it occurs
* Two major types of errors:
    * Recoverable: reasonable to report problem to user/retry operation (e.g.- missing file) 
    * Unrecoverable: always symptoms of bugs (e.g.- array access beyond array length)
* Instead of exceptions, Rust uses the `Result<T, E>` type and `panic!` macro to catch errors

## `panic!` - Unrecoverable Errors
* When called, `panic!` allows for graceful failure
    * print message for user
    * _unwind_ : Rust walks back up the stack and cleans up data from the stack
    * _abort_ : quits execution immediately, leaving the OS to handle memory cleanup

## `Result<T, E>` - Recoverable Errors
* Lots of instances where it doesn't make sense to quit upon error (e.g- trying to open a missing file)
* `Result` is an enum with two variants: `Ok` and `Err`
    ```
    enum Result<T, E> {
        Ok(T),
        Error(E),
    }
    ```
* File reading example:
    ```
    // Quit on failure
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening the file: {:?}", error)
            },
        };
    }
    ```

    ```
    use std::fs::File;

    // Quit on failure, using the unwrap method
    fn main() {
        let f = File::open("hello.txt").unwrap();
    }
    ```

    ```
    use std::fs::File;
    // Same as above but with custom error message
    fn main() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
    ```

    ```
    use std::io::ErrorKind;

    // More informative failure, 
    // tries to detect error type
    fn main() {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            },
        };
    }
    ```

    ```
    use std::fs::File;
    use std::io::ErrorKind;
    // Same as above, but with cleaner syntax then using match
    fn main() {
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }
    ```
* The `?` operator acts as a shorthand for the `match` syntax for functions that return a `Result` type
    ```
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
    ```
* `?` propagates errors upward meaning the `File::open..` line will cause `read_username_from_file` to return an error

## When to `panic!`
*  Unit tests use `unwrap`/`expect` to invoke `panic!`
* Basically, use when failure is not part of the expected functionality
    * e.g. - HTTP requests can fail, don't use there; passing a string to a function requiring a number, use there
* 