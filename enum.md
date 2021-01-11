# Defining an Enum

    enum IpAddrKind {
        V4,
        V6,
    }
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // define a function that takes any IpAddrKind
    fn route(ip_kind: IpAddrKind) {}
    
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
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

We can represent the same concept in a more concise way using just an enum, rather than an enum inside a struct, by putting data directly into each enum variant.

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data. 

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    

# The Option Enum
This enum is Option<T>, and it is defined by the standard library as follows:

    enum Option<T> {
        Some(T),
        None,
    }

<T> means the Some variant of the Option enum can hold one piece of data of any type.
    
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

# Pattern Matching
When the match expression executes, it compares the resulting value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value, execution continues to the next arm, much as in a coin-sorting machine.

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    
## Matching with Option<T>
    
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
## The _ Placeholder
we don’t want to have to list out 0,1,2,3 all the way up to 255. we can use the special pattern _ instead. 
The _ pattern will match any value.

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // The () is just the unit value, so nothing will happen in the _ case.
        _ => (),
    }
