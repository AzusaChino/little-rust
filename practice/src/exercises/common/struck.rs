trait Area {
    fn area(&self) -> u64;
}

#[derive(Debug)] // override trait std::fmt::Display
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Circle {
    r: f64,
}

pub struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

impl AlwaysEqual {
    fn is_equal() -> bool {
        return true;
    }
}

impl Area for Circle {
    fn area(&self) -> u64 {
        (3.14 * self.r) as u64
    }
}

fn create() {
    let _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let _user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // same as #[derive(Debug)] with println!("{:?}", xx);
    dbg!(&_user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn create_color() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}