# Common Programming Concepts
* No real project(s) for this section

## Variables and Mutability
### Mutability
* vars are immutable by default
* once a value is bound to an immutable variable, you can't change that value
    * `error[]: cannot assign twice to immutable variable x`
* make a var mutable by adding `mut` in front
    ```
    let mut x  = 5;
    x = 6;
    ```
* cannot mutate type, only value

### Constants
* `const` keyword
* always immutable
* must be assigned a value (not a result)

### Shadowing
* shadow a previous variable with a new variable
    ```
    fn main() {
        let x = 5;

        let x = x + 1;

        let x = x * 2;

        println!("The value of x is: {}", x);
    }

    ```
* useful when you need to make some transformations but don't want to make the variable mutable

## Data Types
* Two broad categories of types introduced
    * Scalar
    * Compound

### Scalar
* **Integers**: numbers without fractional component

    | Length  | Signed | Unsigned |
    |---------|--------|----------|
    | 8-bit   | i8     | u8       |
    | 16-bit  | i16    | u16      |
    | 32-bit  | i32    | u32      |
    | 64-bit  | i64    | u64      |
    | 128-bit | i128   | u128     |
    | arch    | isize  | usize    |
    
    * Rust defaults to i32

* **Floats**: numbers with fractional component (`f32` or `f64`)
    * Numeric operations
    ```
    fn main() {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;

        // remainder
        let remainder = 43 % 5;
    ```

* **Booleans**: `true` or `false`

* **Characters**: `char` type
    * char literals use single quotes, string literals use double quotes
    ```
    fn main() {
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
    }
    ```

### Compound
* **Tuples**: _A tuple is a general way of grouping together some number of other values with a variety of types into one compound type._
    * Comma separate list inside of parenth
    ```
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
    ```
    * Destructuring: breaking single tuple into many parts
    ```
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x,y,z) = tup;
        println!("y = {}", y);

        // can also access by index
        let first = tup.1
    }
    ```
    * zero indexed

* **Arrays**: many elements, all the same type, fixed length (unlike vectors)
    * Created using square brackets
    ```
    fn main() {
        let a = [1,2,3,4,5];
    }
    ```
    * Good for unchanging groups, e.g. months of year
    ```
    let months = ["January", "February", ....]
    ```
    * Specify type and length
    ```
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    ```
    * Same element, many times
    ```
    // Repeat val = 3, 5 times
    // [3,3,3,3,3]
    let a = [3; 5];
    ```
    * Access by index with square brackets
    ```
    fn main() {
        let a = [1,2,3,4,5];
        let first = a[0];
        let second = a[1];
    }
    ```
## Functions
* Snake case naming convention

### Function Parameters
* Must declare parameter type when writing function
    ```
    fn main() {
        another_function(5, 6);
    }

    fn another_function(x: i32, y: i32) {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }
    ```

### Function Bodies
* **Statements**: instructions that perform action and do not return value
    ```
    let x = 2;
    ```
* **Expressions** evaluate, returning a result of some sort
    * Calling functions/macros etc.
    * No semi-colon
    * Can be part of statements e.g.
    ```
    fn main() {
        let x = 5;
        let y = {
            let x = 3;
            // expression
            x + 1
        };
    }
    ```

### Return Values
* Synonymous with final expression in function
* Type is denoted with `->`
    ```
    fn five() -> i32 {
        5
    }
    ```
## Control Flow
### `if` 
* Branches/arms like the `match` example from Chapter 1
* Condition must be a `boolean`
* No parenths/semicolon in condition 
    ```
    fn main() {
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }
    ```
### `elseif`
* Better to use `match` if there are lots of conditions
```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### `loop`
* Execute repeatedly until told to stop
* exit with `break`
* simple counter
    ```
    fn main() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {}", result);
    }
    ```

### `while`
```
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

### `for`
```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

* simple countdown
    ```
    fn main() {
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }
    ```
