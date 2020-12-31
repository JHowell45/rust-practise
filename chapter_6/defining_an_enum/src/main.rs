fn main() {
	let four = IpAddrKind::V4;
	println!("Four: {:?}", four);
	let six = IpAddrKind::V6;
	println!("Six: {:?}", six);

	route(IpAddrKind::V4);
	route(IpAddrKind::V6);

	let home = IpAddr::new(IpAddrKind::V4, "127.0.0.1");
	println!("Home: {:?}", home);

	let loopback = IpAddr::new(IpAddrKind::V6, "::1");
	println!("Loopback: {:?}", loopback);

	let home = IpAddr2::V4(String::from("127.0.0.1"));
	let loopback = IpAddr2::V6(String::from("::1"));
	println!("Home: {:?}", home);
	println!("Loopback: {:?}", loopback);

	let home = IpAddr3::V4(127,0,0,1);
	let loopback = IpAddr3::V6(String::from("::1"));
	println!("Home: {:?}", home);
	println!("Loopback: {:?}", loopback);

	let m = Message::Write(String::from("hello"));
	m.call();


	// Looking at Option:
	let some_number = Some(5);
	let some_string = Some("a string");

	let absent_number: Option<i32> = None; // Need to specify type as compiler cannot infer the difference.

	println!("some_number: {:?}", some_number);
	println!("some_string: {:?}", some_string);
	println!("absent_number: {:?}", absent_number);

	let _x: i8 = 5;
	let _y: Option<i8> = Some(5);

	// let sum = x + y; // won't compile as Option isn't a valid type for i8 operations!
	
}

#[derive(Debug)]
enum IpAddrKind {
	V4,
	V6,
}

fn route(_ip_kind: IpAddrKind) {}

#[derive(Debug)]
struct IpAddr {
	kind: IpAddrKind,
	address: String,
}

impl IpAddr {
	fn new(kind: IpAddrKind, address: &str) -> IpAddr {
		IpAddr { kind: kind, address: String::from(address) }
	}
}

#[derive(Debug)]
enum IpAddr2 {
	V4(String),
	V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
	V4(u8, u8, u8, u8),
	V6(String),
}

enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColour(i32, i32, i32),
}

// Same as above:
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
	fn call(&self) {
		// do something
	}
}