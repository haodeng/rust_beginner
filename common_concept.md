# Variable
By default, immutable.

    let x = 5;
    x = 6;  # Compile failed

    let mut x = 5;
    x = 6;  # mutable, ok

## Constant
    const MAX_POINTS: u32 = 100_000;

Always immutable, can define at global level

## Shadowing
Declare a new variable with the same name as a previous variable

    let x = 5;
    let x = x + 1;
    let x = x * 2;  # x is 12

mut won't create a new varibale with the same name

    let mut spaces = "   ";
    spaces = spaces.len();  # Compile error: error[E0308]: mismatched types
 
But shadowing is fine, same variable name but has different data types 

    let spaces = "   ";
    let spaces = spaces.len();
