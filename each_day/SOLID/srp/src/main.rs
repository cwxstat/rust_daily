// A struct representing a user
pub struct User {
    pub name: String,
    pub email: String,
    pub age: u32,
}

impl User {
    pub fn new(name: String, email: String, age: u32) -> Self {
        Self { name, email, age }
    }
}

// A module for user management (e.g., creating and updating users)
pub mod user_management {
    use super::User;

    pub fn create_user(name: String, email: String, age: u32) -> User {
        User::new(name, email, age)
    }

    pub fn update_email(user: &mut User, new_email: String) {
        user.email = new_email;
    }

    pub fn update_age(user: &mut User, new_age: u32) {
        user.age = new_age;
    }
}

// A module for user display (e.g., displaying user information)
pub mod user_display {
    use super::User;

    pub fn display_user(user: &User) {
        println!("Name: {}", user.name);
        println!("Email: {}", user.email);
        println!("Age: {}", user.age);
    }
}

fn main() {
    let mut user = user_management::create_user(
        String::from("Alice"),
        String::from("alice@example.com"),
        30,
    );

    user_management::update_email(&mut user, String::from("newalice@example.com"));
    user_management::update_age(&mut user, 31);

    user_display::display_user(&user);
}
