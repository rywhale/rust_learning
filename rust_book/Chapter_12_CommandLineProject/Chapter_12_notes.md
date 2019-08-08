# An I/O Project: Building a Command Line Program
* Going to build a simplified grep
* Major topics
    * organizing code
    * vectors/strings
    * handling errors
    * traits/lifetimes
    * tests

## Accepting Command Line Arguments
* We need the `std::env::args` function to accept arguments
* This returns an iterator (discussed later) and we can call `collect` to turn it into a collection
    ```
    // Bring module into scope
    use std::env;

    fn main() {
        let args: Vec<String> = env::args().collect();
        println!("{:?}", args);
    }
    ```
* Interesting note: the first value in the vector from ^ is actually the program's name (useful for printing messages and stuff)
* We can also assign these arguments to variable
    ```
    use std::env;

    fn main() {
        let args: Vec<String> = env::args().collect();

        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);
    }
    ```
## Reading a File
* `fs::read_to_string` takes a filename and returns a `Result<String>`
    ```
    use std::env;
    use std::fs;

    fn main() {
        let args: Vec<String> = env::args().collect();

        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        println!("With text:\n{}", contents);
    }
    ```

## Refactoring (Modularity/Error Handling)
* Currently, the `main()` function handles parsing argument _and_ reading files
    * this can get confusing if functionality is just piled on
* Error message is currently very uninformative 
* Suggested organizational structure:
    * Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
    * As long as your command line parsing logic is small, it can remain in main.rs.
    * When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

* This leaves main to do:
    * Calling the command line parsing logic with the argument values
    * Setting up any other configuration
    * Calling a run function in lib.rs
    * Handling the error if run returns an error

* main: handles running the program
* lib: handles logic for the task at hand
* lets you test logic of program (can't test main)

### Creating `struct` to hold our Arguments
* Also good to create `struct` with defined types when returning multiple values
    ```
    struct Config {
        query: String,
        filename: String,
    }
    ```
* You can use `.clone()` to allow the struct to take ownership but it creates a full copy in memory

### Creating a Constructor for `Config`
* You can then associate methods with `struct`s using `impl`
    ```
    use std::env;

    fn main() {
        let args: Vec<String> = env::args().collect();

        let config = Config::new(&args);

        // --snip--
    }

    struct Config {
        query: String,
        filename: String,
    }

    // --snip--


    // Using `new` method for Config
    impl Config {
        fn new(args: &[String]) -> Config {
            let query = args[1].clone();
            let filename = args[2].clone();

            Config { query, filename }
        }
    }
    ```
### Fixing Error Handling
* Trying to access an argument by index will cause a panic if the user doesn't provide the requisite number of arguments
* Could do this with `panic!`
    ```
    // --snip--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
    // --snip--
    ```
* But the better strategy is to use the `Result` type
* We can use the `unwrap_or_else` method defined on `Result<T, E>` to customize the user-facing messages
    * If the result is `Ok`-> return the inner value `Ok` is wrapping
    * else run the anonymous function we pass
    ```
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    ```
### Extracting all Logic from Main
* We can further refine the error handling by having the `run` function return a `Result` type
* `Result<(), Box<dyn Error>>` -> The second element is any object with an `Error` trait (more in a later chapter), the `()` simply indicates that this function is used for it's side-effects (not to return elements)
* We can use `?` in the place of `.expect` to return the error value from the current function (rather than panic)
* Since we're returning `()` on success, we only care about detecting an error 
    * can use 
    ```
    if let Err(x) = run(config){
        // -- error action here -- //
    }
    ```

## Developing the Library's Functionality (Plus Testing)
* Test Driven Development
    * Write a test that fails and run it to make sure it fails for the reason you expect.
    * Write or modify just enough code to make the new test pass.
    * Refactor the code you just added or changed and make sure the tests continue to pass.
    * Repeat from step 1!

* STOPPED AT LISTING 12-15