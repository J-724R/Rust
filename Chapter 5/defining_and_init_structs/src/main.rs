fn main() {
    println!("Defining structs");
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user = User {
        active: true,
        username: String::from("Ryuk"),
        email: String::from("ryuk@thebestdog.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("test@test.com");

    fn create_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    println!("Creating instances from other one");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("anotherone@example.com"),
        ..user2
    };

    println!("Using Tuple Structs Without Named Fields to Create Different Types");
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Unit-Like Structs Without Any Fields");
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
