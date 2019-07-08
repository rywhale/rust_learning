# Programming a Guessing Game
* User input 
* Crates/depdendencies
* Basic checking

## Import Library
```
use std::io;
```
## Comments
* Comments in rust use the `//` syntax

## Instantiate Variables
* Use `let` to create immutable vars 
    ```
    let foo = bar;
    ```
* Use `mut` to create mutable vars
    ```
    let mut guess = String::new();
    ```
* Use `::` to use functions associated with `type`
* Sometimes known as a _static method_
    ```
    // 'String' type, 'new' method
    String::new
    ```
* Input/output functionality with `std::io`
    ```
    use std::io

    io::stdin().read_line(...)
    ```

* `&` is used to denote _references_
    * "...gives you a way to let multiple parts of your code access one piece of data without copying it into memory multiple times"
    * references (like vars) are immutable by default

## Failure Handling
* The `Result` types are enumerations (enums)
    * enums have a fixed set of potential values (variants)
    * For `Result`, variants = Ok | Err
* `io::Result` has an `.expect` method which can be used to build out error handling
    * By default, program will crash if `.expect`call returns `Err`
* Rust does a good job of warning you when you fail to properly account for `Result` type (error checking is mandatory bro)

## String Placeholders
* Can use `{}` as a placeholder in strings
    ```
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
    ```

# Generating a Secret Number
## Loading in a Crate
* No built-in random number gen in Rust
* Can have one added to our project by addingit to the `Cargo.toml` file
    ```
    [dependencies]

    rand = "0.3.14"
    ```
* Then use `cargo build` to grab and `cargo update` to update
* Help docs
    ```
    cargo doc --open
    ```
## Ordering by Value
* `std::cmp::Ordering;`
    * enum, variants = Less | Greater | Equal
    * `cmp` = compare
        ```
        match x.cmp(y){
            cond_1 => result_1,
            cond_2 => result_2,
            cond_3 = > result_3,
        }
        ```

## Match
* made up of `arms`
    * format: pattern, result if pattern

## Looping
* `loop` keyword, infinite by default
* `break` to end loop
* `continue` skips to next iteration of loop

## Simple Error Check
* Parsing str -> u32
    ```
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue;
    }
    ```
* The `_` is a catchall 

