fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number == 3 {
        println!("number was three");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    //Because if is an expression, we can use it on the right side of a let statement
    //{} is a block, blocks of code evaluate to the last expression in them
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);


    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;
    //Returning Values from Loops
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; //20
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //Range [1, 4): 1, 2, 3
    for number in 1..4 {
        println!("{}!", number);
    }
}