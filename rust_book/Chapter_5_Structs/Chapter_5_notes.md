# Using Structs to Structure Related Data
* Similar to data attributes in OOPs
* Allows us to define methods/associated functions to sepcify behavior associated with a struct's data
* Like a tuple, except each element is named and you can access by name rather than order
    ```
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    ```
* Must be mutable in order to allow editing
    ```
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    ```
* If parameter names and field names are the same, we can use shorthand
    ```
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    ```

* Duplicating with Struct Update
    ```
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // Same as
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    ```
* Can use tuple struct when naming is unnecessary
    ```
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    ```

## Pretty Printing for Structs
* You can use `#[derive(Debug)]` annotation to create a debug print method for a `struct`
    * use `{:?}` to ref this print method via `println`
    * or `{:#?}` for newline separated (for larger `struct`)
    ```
    #[derive(Debug)]
    struct Rectangle { 
        width: u32,
        height: u32,
    }

    fn main() { 
        let rect1 = Rectangle {
            width: 30,
            height: 50
        };

        println!("rect1 is {:?}", rect1);
    }
    ```

## Methods
* Methods are like functions that are `struct`-specific and always reference `self`
* `impl` - Implementation
