enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum CoinState {
    Penny,
    Nickel,
    Dime,
    // Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // use {} to run multiple lines
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents_state(coin: CoinState) -> u8 {
    match coin {
        CoinState::Penny => 1,
        CoinState::Nickel => 5,
        CoinState::Dime => 10,
        // When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state
        CoinState::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));

    let quater = CoinState::Quarter(UsState::Alaska);
    println!("{}", value_in_cents_state(quater));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none); // Some(5), Some(6), None

    let some_u8_value = 0u8; // 0
    // we don’t want to have to list out 0,1,2,3 all the way up to 255.
    // we can use the special pattern _ instead. The _ pattern will match any value.
    match some_u8_value {
        0 => println!("zero"),
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // The () is just the unit value, so nothing will happen in the _ case.
        _ => (),
    }

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // we could write this in a shorter way using if let. The following code behaves the same
    // The syntax if let takes a pattern and an expression separated by an equal sign. It works the same way as a match
    // Using if let means less typing, less indentation, and less boilerplate code.
    // However, you lose the exhaustive checking that match enforces.
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin = CoinState::Quarter(UsState::Alaska);
    match coin {
        CoinState::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // We can include an else with an if let. It works the same way as a match
    let coin = CoinState::Quarter(UsState::Alaska);
    if let CoinState::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}