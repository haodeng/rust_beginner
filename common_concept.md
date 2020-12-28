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
