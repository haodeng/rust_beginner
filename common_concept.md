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

# Data Types
## Integer Types

    Length	Signed	Unsigned
    8-bit	i8	u8
    16-bit	i16	u16
    32-bit	i32	u32
    64-bit	i64	u64
    128-bit	i128	u128
    arch	isize	usize

Each signed variant can store numbers from -2^(n - 1) to 2^(n - 1) - 1 inclusive, Unsigned variants can store numbers from 0 to 2^n - 1.
the isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

## Other types
check data_type.rs

# Function
Use snake case

    fn another_function() {
        println!("Another function.");
    }

## Function Parameter

    fn main() {
        another_function(5, 6);
    }

    fn another_function(x: i32, y: i32) {
        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }

## Statements and Expressions
Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. 

    let y = 6; // statement
    let x = (let y = 6); // Wrong! Statements do not return values. Therefore, you can’t assign a let statement to another variable

Expressions evaluate to something and make up most of the rest of the code.
Expressions can be part of statements: the 6 in the statement let y = 6; is an expression that evaluates to the value 6. Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, {}, is an expression

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    }; // The block is an expresion, evaluates to 4. x + 1 has no semicolon to the end
