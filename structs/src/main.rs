
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let _user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // Tuple structs: can be destructured and/or elements accessed via '.'
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black is {}, {}, {}\norigin is {}, {}, {}", black.0, black.1, black.2, origin.0, origin.1, origin.2);

    // Unit-Like structs without any fields;
    let _subject = AlwaysEqual;

    // Ownership note
    // let usera = User_s {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };

    // Rectangle
    let w1 = 30;
    let h1 = 50;
    println!(
        "Rect {} x {}: The area of the rectangle is {} square pixels.",
        w1, h1, area_wh(w1, h1)
    );

    let rect1 = (30, 50);
    println!(
        "Rect {:?}: The area of the rectangle is {:?} square pixels.",
        rect1, area_tup(rect1)
    );

    let rect1 = Rectangle {
        w: 30,
        h: 50,
    };

    println!(
        "Rect struct Rectangle {:?}: The area of the rectangle is {} square pixels.",
        &rect1, area_rect(&rect1)
    );

}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.w * rect.h
}

#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

fn area_wh(w: u32, h: u32) -> u32 {
    w * h
}

fn area_tup(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// Ownership note
// struct User_s {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Struct update syntax: to create a different User struct that's similar but not identical.

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;