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
* Implemented as a collection of bytes + methods for handling/interpreting those bits
* `&str` - String _slice_
* `String` - String _literal_
* Both ^ are UTF-8 encoded
* Creating a new string
    ```
    let mut s = String::new();
    ```
* Init string with value
    ```
    // Either using another variable
    let data = "test string";
    let s = data.to_string();

    // Or the string literal
    let s = "test string".to_string();

    // Or an equivalent function
    let s = String::from("test string");
    ```
* Appending to String
    ```
    // push_str for strings
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    // push for single char
    let mut s3 = String::from('lol');
    s3.push('l')
    ```
* String concat using `+`
    ```
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("!");

    // Takes ownership of s1
    let s = s1 + "-" + &s2 + "-" + &s3
    ```
* String concat using `format!`
    ```
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("!");

    // Doesn't take ownership
    let s = format!("{}-{}-{}", s1, s2, s3);
    ```
* **Rust strings don't support indexing**
* A String is actually a wrapper over `Vec<u8>`
* Thus, a string's length = the number of bytes taken to encode it
    * This can differ depending on the characters
    * e.g. 
        ```
        // This  = 4
        let len = String::from("Hola").len();

        // This should be 12 (in theory)
        // but the actual bytes required  = 24
        let len = String::from("Здравствуйте").len();
        ```
* String iteration
    * use `chars`
    ```
    for c in "dogboi".chars() {
        println!("{}", c);
    }
    ```

## Hash Maps
* Key,Value pairs (similar to dictionaries/lists in other langs)
* Stored in the heap (like vectors)
* Keys must be same type, values must be same type e.g.- String, i32
* Init and add items individually
    ```
    // Bring in library
    use std::collections::HashMap;
    // Create 
    let mut petscores = HashMap::new();
    // Use insert to add elements
    petscores.insert(String::from("Dog"), 3);
    petscores.insert(String::from("Cat"), 4);

    ```
* Init and add items from tuples
    ```
    use std::collections::HashMap;
    let pets = vec![String::from("Dog"), String::from("Cat")];
    let init_scores = vec![3, 4];

    // Specify HashMap format for `collect`
    let scores: HashMap<_,_> = pets.iter().zip(init_scores.iter()).collect();
    ```
    * `collect` works with many data structures, hence the type annotation `HashMap<_,_>`
    * Values with `Copy` trait (i32) are copied into HashMap
    * Values without (String) will be moved and hashmap becomes owner
* Accessing values
    ```
    use std::collections::HashMap;

    let mut pets = HashMap::new();

    pets.insert(String::from("Dog"), 3);
    pets.insert(String::from("Cat"), 4);

    let fav_anim = String::from("Cat");

    let fav_score = pets.get(&fav_anim);
    ```
* Re-assignment by overwriting using `insert`
    ```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
    ```
* Conditional assignment (only if not assigned yet) using `or_insert`
    ```
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    ```
* Values iteration
    ```
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Split on whitespace and iterate
    // (creates mutatable ref to each word)
    for word in text.split_whitespace() {

        // If doesn't exist in hashmap, add and set val to 0
        let count = map.entry(word).or_insert(0);

        // Else, add one to existing val
        *count += 1;
    }

    println!("{:?}", map);
    ```
    * The asterisk before count dereferences it and brings it into the current scope (the for loop)
    * Then falls out of scope at the end of the loop