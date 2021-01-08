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
