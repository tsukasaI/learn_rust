#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    activate: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        name: String::from("a123"),
        email: String::from("someone@example.com"),
        activate: true,
        sign_in_count: 1,
    };
    let mut user1 = User {
        name: String::from("a123"),
        email: String::from("someone@example.com"),
        activate: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@e.com");
    println!("{:#?}", user1);

    let user2 = build_user(String::from("a123@e.com"), String::from("a123"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect);
}

fn build_user(email: String, name: String) -> User {
    User {
        name,
        email,
        activate: true,
        sign_in_count: 1,
    }
}
