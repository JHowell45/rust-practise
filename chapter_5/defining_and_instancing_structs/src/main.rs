fn main() {
    let mut user1 = User {
		email: String::from("john.smith@hotmail.com"),
		username: String::from("john_smith"),
		sign_in_count: 1,
		active: true,
	};

	user1.email = String::from("john.smith@gmail.com");

	let user2 = User {
		email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
	};

	// More succinct version of the above:
	let user2 = User {
		email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
	};

	let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

struct Colour(i32, i32, i32); // Tuple struct
struct Point(i32, i32, i32);

fn build_user(username: String, email: String) -> User {
	User {
		username, // variable shares same name as struct attr, so no need to reference.
		email, // same as above.
		active: true,
        sign_in_count: 1,
	}
}