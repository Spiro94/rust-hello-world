// Structs

fn main() {
    // Tuple
    let rec: (i32, i32) = (200, 500);

    // Struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("scarkov"),
        email: String::from("scarkov@gmail.com"),
        sign_in_count: 0,
    };

    user1.email = String::from("anotheremail@mail.com");
    println!("User email is {}", user1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    let user2 = User {
        email: String::from("another@m.com"),
        ..user1
    };

    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
