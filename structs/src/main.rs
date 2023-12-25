/*
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    /*
        let mut user1 = User {
            username: String::from("himanshubhardwaz"),
            email: String::from("himanshu76200@gmail.com"),
            sign_in_count: 1,
            active: true,
        };

        let name = user1.username;
        user1.username = String::from("newusername");

        let user2 = build_user(
            String::from("himanshu76200@gmail.com"),
            String::from("himanshunew"),
        );
    */

    let rect1 = Rectangle {
        height: 20,
        width: 40,
    };

    let rect2 = Rectangle {
        height: 10,
        width: 20,
    };

    println!("Area of rect 1: {}", rect1.area());
    println!("Area of rect 2: {}", rect2.area());

    if rect1.can_hold(&rect2) {
        println!("Rectangle 1 can hold Reactangle 2");
    }

    if rect2.can_hold(&rect1) {
        println!("Rectangle 2 can hold Reactangle 1");
    }
}

/*
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
*/
