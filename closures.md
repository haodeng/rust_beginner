# Closures: Anonymous Functions that Can Capture Their Environment
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. 
You can create the closure in one place and then call the closure to evaluate it in a different context. 
Unlike functions, closures can capture values from the scope in which they’re defined. 

    // if we had more than one parameter, we would separate them with commas, like |param1, param2|
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        
        // The value returned from the last line in the closure body (num) 
        // will be the value returned from the closure when it’s called, 
        // because that line doesn’t end in a semicolon; just as in function bodies.
        num
    };
    
    // We call a closure like we do a function
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

## Closure Type Inference and Annotation
Closures don’t require you to annotate the types of the parameters or the return value like fn functions do.
These are all valid definitions that will produce the same behavior when they’re called.

    // a function definition
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    
    // a fully annotated closure definition.
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    
    // removes the type annotations from the closure definition
    let add_one_v3 = |x| { x + 1 };
    
    // removes the brackets, which are optional because the closure body has only one expression.
    let add_one_v4 = |x| x + 1 ;

Calling the closures is required for add_one_v3 and add_one_v4 to be able to compile because the types will be inferred from their usage.

Closure definitions will have one concrete type inferred for each of their parameters and for their return value.

    // Attempting to call a closure whose types are inferred with two different types
    // Note that we haven’t added any type annotations to the definition: 
    // if we then try to call the closure twice, 
    // using a String as an argument the first time and a u32 the second time, we’ll get an error.
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // expected struct `String`, found integer
    let n = example_closure(5);
    
The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. 
Those types are then locked in to the closure in。


## Storing Closures Using Generic Parameters and the Fn Traits
To make a struct that holds a closure, we need to specify the type of the closure, because a struct definition needs to know the types of each of its fields. 
The Fn traits are provided by the standard library. All closures implement at least one of the traits: Fn, FnMut, or FnOnce. 

    // The trait bounds on T specify that it’s a closure by using the Fn trait. 
    // Any closure we want to store in the calculation field must have one u32 parameter 
    // (specified within the parentheses after Fn) and must return a u32 (specified after the ->).
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }
    
  The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as memoization or lazy evaluation.
  
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
                    // the expensive calculation will be run a maximum of once.
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    
    fn generate_workout(intensity: u32, random_number: u32) {
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
