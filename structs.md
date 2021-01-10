# Structs
## Defining and Instantiating Structs
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

## Using the Field Init Shorthand when Variables and Fields Have the Same Name

    // having to repeat the email and username field names and variables is a bit tedious.
    // there’s a convenient shorthand!
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn build_user_simple(email: String, username: String) -> User {
        User {
            // Because the email field and the email parameter have the same name, we only need to write email rather than email: email
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    
## Creating Instances From Other Instances With Struct Update Syntax

    let user5 = User {
        email: String::from("another5@example.com"),
        username: String::from("anotherusername5"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    
    // The syntax .. specifies that the remaining fields not explicitly set
    // should have the same value as the fields in the given instance.
    let user6 = User {
        email: String::from("another6@example.com"),
        username: String::from("anotherusername6"),
        ..user1
    };

# Tuple structs
Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // black and origin values are different types, because they’re instances of different tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


# Print Debug info
add the annotation #[derive(Debug)] just before the struct definition

    // print out debugging information
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1 is {:?}", rect1); // debug info. `{:?}` (or {:#?} for pretty-print)
    }

# Define Methods
To define the function within the context of Rectangle, we start an impl (implementation) block.

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    

