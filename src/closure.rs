use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

// Attempting to call a closure whose types are inferred with two different types
fn not_working() {
    let example_closure = |x| x;

    // The first time we call example_closure with the String value,
    // the compiler infers the type of x and the return type of the closure to be String.
    let s = example_closure(String::from("hello"));

    // error[E0308]: mismatched types, expected struct `String`, found integer
    // let n = example_closure(5);
}

// Defining a Cacher struct that holds a closure in calculation and an optional result in value
struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

// The caching logic of Cacher
impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout_cached(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn capture_env() {
    let x = 4;

    // even though x is not one of the parameters of equal_to_x,
    // the equal_to_x closure is allowed to use the x variable that’s defined in the same scope
    // that equal_to_x is defined in.
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

fn main() {
    // generate_workout(1, 2);
    generate_workout_cached(1, 2);

    capture_env();
}