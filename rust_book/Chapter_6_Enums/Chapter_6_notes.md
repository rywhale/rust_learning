# Enums and Pattern Matching
* **enums** allow you to define a type by enumerating its possible values

## IP Address Example
* IP addresses can be version 4 or version 6
    ```
    enum IpAddrKind { 
        V4,
        V6,
    }
    ```
* Struct version
    ```
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    ```
* Enum version
    ```
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    ```
* Enum provides greater flexibility in that each variant can have different types and amounts of associated data (e.g.- IP address doesn't have to be string in both types)
    ```
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    ```
## `Option` Example
* The `Option` type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing
* Instead of having a `null` value, Rust implements an enum that can encode the concept of a value being present or absent
    ```
    enum Option<T> {
        Some(T),
        None,
    }
    ```

## Control Flow with `match`
* **match**: allows you to compare a value against a series of patterns and then exectue code based on which pattern matches
    * _Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution._

    ```
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
    ```
* `_` placeholder
    * will match any value, can be used after arms in match to match all cases not matched by previous arms
    ```
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    ```

## Control Flow with `if let`
* Best used for single pattern condition matches (you only care about one pattern)
    ```
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    ```
