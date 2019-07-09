# Project Management w/Packages, Crates and Modules
* **Packages**: A Cargo feature that lets you build, test, and share crates
* **Crates**: A tree of modules that produces a library or executable
* **Modules** and **use**: Let you control the organization, scope, and privacy of paths
* **Paths**: A way of naming an item, such as a struct, function, or module

## Packages and Crates
* A crate is a binary or library
* Crate root is a source file that the Rust compiler starts from
* Package is one or more crates that provide a set of functionality (contains a `Cargo.toml` file)
* Cargo assumes that `src/main.rs` is the crate root for binary crates, `src/lib.rs` for library crates
* **Library crate** -> code to be used by other Rust code
* **Binary crate** -> code to be made into an executable


## Modules, Control Scope and Privacy
* _Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or whether it’s an internal implementation detail and not available for outside use (private)._
* Create a new library with
    ```
    cargo new --lib LIBNAME
    ```
* Modules create with the `mod` keyword
* In Rust, all items (functions, methods, structs, enums, modules, and constants) are private by default 
    * can change this using the `pub` keyword 
* Privacy cascades downwards
    * Access to a parent != access to a child
    * Acess to a child == access to a parent


### Paths
* A path can take two forms:
    * An absolute path starts from a crate root by using a crate name or a literal crate.
    * A relative path starts from the current module and uses self, super, or an identifier in the current module.
* Both types are separated with `::`

### Scope
* You can bring paths into scope with `use` (avoids long import statements)
        ```
        mod front_of_house {
            pub mod hosting {
                pub fn add_to_waitlist() {}
            }
        }

        use crate::front_of_house::hosting;

        pub fn eat_at_restaurant() {
            hosting::add_to_waitlist();
            hosting::add_to_waitlist();
            hosting::add_to_waitlist();
        }
        ```
* Functions
    * keep parent module in path `parent::function`
* Enums/structs/etc 
    * full path `element`
    * except if namespace clashes, then `module::element`


### Renaming 
* You can use `as` keyword
    ```
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        // --snip--
    }

    fn function2() -> IoResult<()> {
        // --snip--
    }
    ```
* Re-export names with `pub use`
    ```
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
    ```

### Nesting Paths (Bulk imports)
* Multiple fns/modules
    ```
    use std::{cmp::Ordering, io};
    ```
* Self + submodules
    ```
    //Instead of 
    use std::io;
    use std::io::Write;

    //Do
    use std::io::{self, Write};
    ```
* Everything
    ```
    use std::collections::*;
    ```
    
### Restaurant Example
* Front/back of house in restaurant each with different roles/tasks
* Nesting modules creates _families_
    * Children mods, parent mods, sibling mods etc.
* Functions defined in the same crate can be accessed using the `crate` keyword at the start of the path
    * `crate::front_of_house::...`
