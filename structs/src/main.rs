#![allow(unused)]
fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: usize,
    }

    let user1: User = User { 
        active: true,
        username: String::from("tapphughesn"),
        email: String::from("someone@example.com"),
        sign_in_count: 10,
    };

    struct TupleLike(i32, String, str);

    struct UnitLike;
}

