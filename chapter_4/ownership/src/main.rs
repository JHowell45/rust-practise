fn main() {
	let stack_s = "hello"; // variable is a string literal so stored on the stack.
	let mut heap_s = String::from(stack_s); // variable is of String type (variable type) so is stored on the heap.

	println!("String value: {}", heap_s);
	heap_s.push_str(" world");
	println!("String value: {}", heap_s);

	let x = 5; // binds 5 to variable/owner 'x'.
	let y = x; // makes copy of the value in x, 5, to bind to the variable/owner y.

	println!("x: {}", x);
	println!("y: {}", y);

	let x = 7;

	println!("edited x: {}", x);
	println!("y: {}", y);

	let s1 = String::from("Hello!"); // allocates memory on the heap and adds the ptr, length, and allocated memory for this variable to the stack.
	let s2 = s1; // makes a copy (it's considered being "moved") of the ptr, length, and allocated memory for the value s1.

	println!("s2: {}", s2); // Can't print s1 as it's considered to be out of scope already as ownership is passed to s2.

	let s1 = String::from("Hello!");
	let s2_clone = s1.clone(); // makes a "deep" copy the s1 data, cloning the heap data and not just the stack data.

	println!("s1: {}, s2 clone: {}", s1, s2_clone);


	let s = String::from(stack_s);
	takes_ownership(s);
	// println!("s: {}", s); // doesn't work as s no longer exists as the ownership was passed to the scope of 'takes_ownership'.

	let x = 5;
	makes_copy(x);
	println!("x: {}", x);


	let s1 = gives_ownership(); // receives ownership of the local heap variable within the 'gives_ownership' function.

	let s2 = String::from(heap_s);

	let s3 = takes_and_gives_ownership(s2); // takes ownership of s2 in the function and returns the heap variable ownership back to s3.
}


fn takes_ownership(some_string: String) {
	println!("Some string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("Some Integer: {}", some_integer);
}

fn gives_ownership() -> String {
	let some_string = String::from("Hello!");
	some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
	some_string
}