struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct User2<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("muhlickha"),
        email: String::from("malika@google.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("anothermalikaemail@google.com"),
        ..user1
    };

    user1.email = String::from("malika2@google.com");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;

    let subject = AlwaysEqual;
}
