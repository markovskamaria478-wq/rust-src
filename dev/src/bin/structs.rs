#![allow(unused)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,
        sign_in_count: 1,
    }
}

struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = build_user(
        "someone@example.com".to_owned(),
        "someusername123".to_owned(),
    );
    let _user2 = User {
        email: "user2email@example.com".to_owned(),
        ..user1
    };

    // println!("{}", user1.username); won't compile; user1.username was moved because String doesn't implement Copy trait

    let origin = Point(0, 0, 0);
    let Point(_x, _y, _z) = origin;

    let example = AlwaysEqual;
}