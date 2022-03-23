use colored::*;  // https://stackoverflow.com/a/69981450, https://lib.rs/crates/colored

struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool
}

fn build_user(email: String, username: String) -> User {
    /*
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
    */

    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {

	let mut user1 = User {
		email: String::from("user@abc.com"),
		username: String::from("user"),
		active: true,
		sign_in_count: 1,
	};

	println!("user1 = {}", user1.username.green());

	user1.username = String::from("IU");
	println!("\nUpdated\nuser1 = {}", user1.username.yellow());

    let user2 = build_user(String::from("jpark@abc.com"), String::from("jpark"));
	println!("\nAdded user by fn\nuser2 = {}", user2.username.green());

	let user3 = User {
		email: String::from("star@abc.com"),
		username: String::from("star"),
		active: user1.active,
		sign_in_count: user1.sign_in_count,
	};
	println!("\nAdded user by user1\nuser3 = {}", user3.username.green());

	let user4 = User {
		email: String::from("moon@abc.com"),
		username: String::from("moon"),
        ..user1
	};
	println!("\nAdded user by user1\nuser4 = {}", user4.username.green());

    // tuple structure
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // different type, Color != Point
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
