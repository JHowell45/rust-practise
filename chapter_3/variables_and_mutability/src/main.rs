fn main() {
	// let x = 5; doesn't work as the value isn't mutable be default
	let mut x = 5;
	println!("The value of x is: {}", x);
	x = 6;
	println!("The value of x is: {}", x);

	const MAX_POINTS: u32 = 100_000;
	println!("The value of const MAX_POINTS is : {}", MAX_POINTS);

	// shadowing:
	let x = 5;
	println!("The value of x is: {}", x);
	let x = x + 1;
	println!("The value of x is: {}", x);
	let x = x * 2;
	println!("The value of x is: {}", x);
	// Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used.
	// Good for changing variables type.
}
