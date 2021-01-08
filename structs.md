# Structs

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("{}", user1.email); // To get value, use dot notation

To set value, struct need to be mutable

    // the entire instance must be mutable
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
