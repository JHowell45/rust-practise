fn main() {
    let some_u8_value = Some(0u8);
	
	match some_u8_value { // version 1
        Some(3) => println!("three"),
        _ => (),
	}
	println!("some_u8_value: {:?}", some_u8_value);

	if let Some(3) = some_u8_value { // version 2
		println!("three");
	}
	println!("some_u8_value: {:?}", some_u8_value);

	// version 2 is identical to the above match condition, but more concise.

}
