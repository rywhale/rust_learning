# 1. Getting Started
## Install
* Install Rust for windows using [this](https://www.rust-lang.org/tools/install)
* Check version to ensure proper install 
    ```
    rustc --version
    ```

## Hello World
* Rust files have a `.rs` extension
* Use underscores to separate words in script file names
    * e.g.- `hello_world.rs`
* Running scripts
    ```
    rustc main.rs
    ```
    * Produces a `.exe` file on Windows
    ```
    .\main.exe
    ```
* Functions declared with format
    ```
        fn functionname() {

        }
    ```
* Macros have `!`, functions don't
    * `println!`
* Four space idents
* Semi-colon closers

### Compiling vs Running
`rustc` is used to compile
```
rustc main.rs
```
Compilation produces an executable file which can be run without a Rust install.

## Hello Cargo
* Dependency manager for projects
* Init a new cargo project via
    ```
    cargo new project_name
    ```
* Creates a `.TOML` file with metadata related to dependencies, rust vers, author, etc.
* Directory layout
    ```
    -project_name
        --Cargo.toml
        --src
            ---main.rs
    ```
* Build cargo project
    ```
    cargo build
    ```
    * only actually rebuilds if files have changed
    * creates directory `project_name/target/debug` to house exe
* Build + exectute
    ```
    cargo run
    ```
* Check code
    ```
    cargo check
    ```
* Build for release (`project_name/target/release`)
    ```
    cargo build --release
    ```