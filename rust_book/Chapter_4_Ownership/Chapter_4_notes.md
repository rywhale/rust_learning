# Ownership
* _Rust's most unique feature_
* All programs have strats for managing memory while running
* Some programs use garbage collection to look for unused memory as program runs
    * in others this must be done explicity by the programmer
* Rust uses a different approach: programs have ownership rules that the compiler can check at compile time

## Stack vs Heap
* both are parts of memory available to program at run time
* Stack
    * _Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top._
    * stores values in order it gets them, removes in opposite order
        * _last in, first out_
    * all data stored here must have known, fixed size'
    * When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack
* Heap
    * _Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you’ve been seated to find you._
    * less organized
    * the OS finds an empty spot in the heap that is big enough for the requested data in the heap and marks it as 'in use'
        * this returns a _point_ which is the memory address of that location
        * called _allocating on the heap_
    * pointers can be stored on the stack
* Push to the stack  = faster (no allocation time)

## Ownership Rules
* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

### Variable Scope
* The `String` type (example)
    * allocated on the heap, can store an unknown amount of text 
    * convert string literal -> string
    ```
    let s = String::from("hello");
    ```
    * String can be mutable
    ```
    let mut s = String::from("hello");
    // Convert literal string and add to end
    s.push_str(", world!");
    ```
    * This is because String type is allocated to heap
    * By extension this means String has to have ways of allocating/returning memory at runtime
    * Allocation -> `String::from`
    * In Rust, the memory is automatically returned once the variable is out of scope
    * The `}` at the end of a scoped section calls the `drop` function to remove the value from memory

## Data Interaction
### Moving
* Both values are pushed to stack
    ```
    let x = 5;
    let y = x;
    ```
* Strings
    ```
    let s1 = String::from("hello");
    ```
    * s1 has three components
        1. ptr - pointer to memory holding contents of string
        2. len - memory in bytes being used
        3. capacity - memory in bytes the String has been given by OS
    * Left-stack, Right-heap: ![](https://doc.rust-lang.org/book/img/trpl04-01.svg)
    ```
    let s1 = String::from("hello");
    let s2 = s1;
    ```
    * Left-stack, Right-heap: ![](https://doc.rust-lang.org/book/img/trpl04-02.svg)
    * Assigining s1's value to s2 **invalidates s2**, called a `move`
        * `s1` _was moved into_ `s2`
        * ![](https://doc.rust-lang.org/book/img/trpl04-04.svg)


### Cloning
* Just like `move` except for the _heap data is copied_
```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

### Copying (Stack-only data)
* Only valid for types that have a known size at compile time and are therefore stored entirely on the stack (copies are quick to make)
* types (like integers) have a `Copy` trait
* any group of simple scalar values can have `Copy`
    * All the integer types, such as u32
    * The Boolean type, bool, with values true and false
    * All the floating point types, such as f64
    * The character type, char
    * Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not

### Ownership and Functions
* Passing a variable to a function will move or copy, just as assignment does
```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Referencing and Borrowing
* It's possible to `reference` a variable without taking ownership
    ```
    fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    ```
    * The `&s1` syntax lets us create a reference that refers to the value of `s1` but does not own it
    * Also use `&` in function call to indicate that `s` is a reference
    * Having references as function params is called `borrowing`
        * immutable my default

### Mutable References
* Can make mutable, however each scope can only have one mutable reference for a piece of data and cannot mix mutable/immutable for the same piece of data
    ```
    fn main() {
        let mut s = String::from("hello");

        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    ```
* This fails due to trying to mutate multiple times in the same scope
    ```
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    ```
* At any given time, you can have either one mutable reference or any number of immutable references
* References must always be valid

### Slice Type
* _...let you reference a contiguous sequence of elements in a collection rather than the whole collection_
* Getting last index of first word in string with for loop     
    ```
    fn first_word(s: &String) -> usize {
        // Convert to array of bytes
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            // Look for spaces (b ' ' = byte literal, space character)
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }
    ```
* String slicing 
    ```
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    ```
    ![](https://doc.rust-lang.org/book/img/trpl04-06.svg)

    * indexing by range/length
        ```
        let s = String::from("hello");

        let len = s.len();

        let slice = &s[0..len];
        let slice = &s[..];
        ```
    * re-write fn from beginning (note: `&str` as type "string slice")
        ```
        fn first_word(s: &String) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
        ```