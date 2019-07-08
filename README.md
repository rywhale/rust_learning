# About
Assorted resources/notes gathered while learning Rust.

# Resource List
* Rust Book ([link to book](https://doc.rust-lang.org/book/))
    * [Chapter 1 notes](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_1_GettingStarted/Chapter_1_notes.md)
    * [Chapter 2 notes](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_2_GuessingGame/Chapter_2_notes.md)
    * [Chapter 3  notes](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_3_CommonProgrammingConcepts/Chapter_3_notes.md)
    * [Chapter 4 notes](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_4_Ownership/Chapter_4_notes.md)
    * [Chapter 5 notes]()
* Rust by example ([link to book](https://doc.rust-lang.org/stable/rust-by-example/))
* Rust Cookbook ([link to book](https://rust-lang-nursery.github.io/rust-cookbook/))

# Windows + Antivirus Install Issues
Having OS prvilege issues installing via `rustup` on Windows? I had issues due to privilege/antivirus related issues. You can get around this by changing the default install directories for the Rust toolchain/utilities in your Windows environment settings. 

Workaround: 
* Before you run `rustup-init`, edit your Windows Environment settings to include the following:
    * `CARGO_HOME` -> A location (not on the root C drive)
    * `RUSTUP_HOME` -> A location (not on the root C drive)
* Run `rustup-init` as usual and it will respect the paths you set above