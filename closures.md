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
