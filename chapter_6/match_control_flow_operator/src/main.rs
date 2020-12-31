fn main() {
	let v = value_in_cents(Coin::Quarter(UsState::Alabama));
	println!("v: {}", v);

	let five = Some(5);
    let six = plus_one(five);
	let none = plus_one(None);
	println!("five: {:?}", five);
	println!("six: {:?}", six);
	println!("none: {:?}", none);
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
}


enum Coin {
    Penny,
    Nickel,
	Dime,
	// Quarter // replaced with the one below to allow for states to be specified for the coin.
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	} // can also use '_' as a wildcard placeholder for other options instead of listing them all out.
}