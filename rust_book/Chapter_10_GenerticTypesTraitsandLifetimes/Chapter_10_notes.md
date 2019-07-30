**note to self** : revisit this section

# Generic Types, Traits and Lifetimes
* **generics** are abstract stand-ins for concrete types or other properties
    * `Vec<T>`
    * `Result<T, E>`
    * `Option<T>`
* **traits** define behavior in a generic way
* **lifetimes** give information about how references relate to each other

## Function Extraction
* Identify duplicate code
* Extract into body of another function and specify inputs/outputs in function signature
* Update duplicated code to call function instead
* e.g.
    * Before
    ```
    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }
    ```

    * After
    ```
    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let result = largest(&number_list);
        println!("The largest number is {}", result);
    }
    ```

## Generic Data Types
### In Function Definitions
* Let's say we want to recreate a function for calculating the largest element in a list for both `i32` and `char` types
* Instead of specifying input types in the function signature, we can use a generic
    ```
    fn largest <T>(list: &[T]) -> T {
        ...
    }
    ```
    * read as "the function largest is generic over some type `T`"

### In Struct Definitions
* e.g.- One generic (fails if provided different types)
    ```
    struct Point<T> {
        x: T,
        y: T,
    }

    fn main() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
    }
    ```
* multiple generics (accounts for possibly different types)
    ```
    struct Point<T, U> {
        x: T,
        y: U,
    }

    fn main() {
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }
    ```

### In Enum Definitions
* Already seen some examples
    ```
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```

### In Method Definitions
* Methods can also be crafted to accept generics
    ```
    struct Point<T> {
        x: <T>,
        y: <T>,
    }

    // Method to return x value from Point struct
    // note: generic, will return any type
    impl <T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10};
        println!("p.x = {}", p.x());
    }
    ```
* This can be refined to be type specific
    ```
    struct Point<T> {
        x: <T>,
        y: <T>,
    }

    impl Point<f32>{
        fn fake_fun(&self) -> f32 {
            *self.x.powi(2).sqrt()
        }
    }
    ```
    * This creates a `fake_fun` method for Point struct _only when_ Point is type `<f32>`
        * allows us to specify methods for handling various type possibilities

## Traits: Defining Shared Behavior
* A trait tells us about the functionality of a specific type/how types share functionality
* A type's behavior consists of methods we call on that type
* Trait definitions are a way of grouping method signatures together to define a set of behaviors
### Defining a Trait
* Imagine we have two structs:
    * NewsArticle: holds news article text, author, etc.
    * Tweet: holds tweet text, tweet author, etc.
* We want to create a trait for both of these structs called `Summary` that provides a summary regardless of the source/struct
* We have to first define a trait (`pub trait foo`) then implement (`impl foo for bar`)

```
pub trait Summary { 
    fn summarise(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```