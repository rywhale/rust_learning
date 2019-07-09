# Common Collections
* Rust contains built-in data structures (_collections_)
* Store on the heap (unlike tuples/arrays)
* Three common types (there are others)
    * A **vector** allows you to store a variable number of values next to each other
    * A **string** is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth
    * A **hash map** allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map

## Vectors
* `Vec<T>`
* only values of same type
* Create new vector (notation is optional, Rust will try to infer type of vector elements)
    ```
    let v: Vec<i32> = Vec::new();
    ```
* Short-hand macro is `vec!`
    ```
    let v = vec![1,2,3];
    ```
* Add elements with `push`
    ```
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    ```
* Acessing elements
    ```
    let v = vec![1,2,3,4,5];
    // Square bracket indexing
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Using get
    match v.get(2) {
        Some(third) => println!("Third element is {}", third),
        None => println!("No third element),
    };
    ```
* Iteration
    ```
    // Immmutable
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //Mutable (changing vector with loop)
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    ```
* Using `enum` to circumvent the only one type rule for vectors
    ```
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let rol = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue)),
        SpreadsheetCell::Float(10.12),
    ];
    ```
## Strings
