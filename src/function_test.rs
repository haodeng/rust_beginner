fn five() -> i32 {
    5 //Expresion, evaluates to 5, no semicolon to the end. If add semicolon (changing it from an expression to a statement), compile Error
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();
    let y = plus_one(5);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}