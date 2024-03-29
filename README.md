# About
Assorted resources/notes gathered while learning Rust.

# Resource List
* Rust Book ([link to book](https://doc.rust-lang.org/book/))
    * My notes
        * [Chapter 1](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_1_GettingStarted/Chapter_1_notes.md)
        * [Chapter 2](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_2_GuessingGame/Chapter_2_notes.md)
        * [Chapter 3](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_3_CommonProgrammingConcepts/Chapter_3_notes.md)
        * [Chapter 4](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_4_Ownership/Chapter_4_notes.md)
        * [Chapter 5](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_5_Structs/Chapter_5_notes.md)
        * [Chapter 6](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_6_Enums/Chapter_6_notes.md)
        * [Chapter 7](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_7_ProjectManagement/Chapter_7_notes.md)
        * [Chapter 8](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_8_CommonCollections/Chapter_8_notes.md)
        * [Chapter 9](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_9_ErrorHandling/Chapter_9_notes.md)
        * [Chapter 10](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_10_GenericTypesTraitsandLifetimes/Chapter_10_notes.md)
        * [Chapter 11](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_11_WritingAutomatedTests/Chapter_11_notes.md)
        * [Chapter 12](https://github.com/rywhale/rust_learning/blob/master/rust_book/Chapter_12_CommandLineProject/Chapter_12_notes.md)

* Rust by example ([link to book](https://doc.rust-lang.org/stable/rust-by-example/))
* Rust Cookbook ([link to book](https://rust-lang-nursery.github.io/rust-cookbook/))

# Project Ideas 
## Simple
* Unit/datum conversion tool
* Hangman/snake/battleship/etc.

## Less Simple
* TCP/IP client (GOES feed?)
* SOAP client
* Synthesizer 

# Windows + Antivirus Install Issues
Having OS privilege issues installing via `rustup` on Windows? I had issues due to privilege/antivirus related issues. You can get around this by changing the default install directories for the Rust toolchain/utilities in your Windows environment settings. 

Workaround: 
* Before you run `rustup-init`, edit your Windows Environment settings to include the following:
    * `CARGO_HOME` -> A location (not on the root C drive)
    * `RUSTUP_HOME` -> A location (not on the root C drive)
* Run `rustup-init` as usual and it will respect the paths you set above