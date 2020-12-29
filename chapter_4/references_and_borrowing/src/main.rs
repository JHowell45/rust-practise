fn main() {
	let s = String::from("Hello");
	let len = calculate_length(&s); // passing the reference to 's' to the function and not moving the variable in.
	println!("l: {}", len);

	let mut s = String::from("Hello");
	// change(&s); // doesn't work as even though the String is mutable, the reference to it is not.

	println!("s: {}", s);
	change(&mut s);
	println!("s: {}", s);

	// ONLY ALLOWED A SINGLE MUTABLE REFERENCE IN A SINGLE SCOPE!!
	// let mr1 = &mut s;
	// let mr2 = &mut s;
	// println!("mr1: {}, mr2: {}", mr1, mr2);

	// Can fix by placing one of the references within another scope:
	{
		let mr1 = &mut s;
		println!("r: {}", mr1);
	}
	let mr2 = &mut s;
	println!("r: {}", mr2);

	let mut s = String::from("hello");
	let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1: {} and r2: {}", r1, r2); // If this is missing there will be an issue with the mut ref later as you can't have a read and a read/write ref.
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("r3: {}", r3);
}


fn calculate_length(s: &String) -> usize { // the & symbol is used for getting the reference (ptr I think) for the specific variable and not just move it all into the function.
	s.len()
	// function doesn't have ownership of s so doesn't 'drop' the variable from the heap.
}

fn change(s: &mut String) {
	s.push_str(" world!");
}
