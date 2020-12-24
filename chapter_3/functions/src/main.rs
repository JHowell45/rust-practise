fn main() {
	println!("Hello, world!");
	
	another_function();
	display_number(5);
	display_x_and_y(5,6);

	// statement:
	let x = 5;

	// statement -> expression:
    let y = {
        let x = 3;
        x + 1
	};
	println!("Value of x: {}", x);
	println!("Value of y: {}", y);

	let x = five();
	println!("The value of x: {}", x);

	let x = plus_one(5);
	println!("The value of x: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn display_number(x: i32) {
	println!("The value for 'x' is: {}", x);
}

fn display_x_and_y(x: i32, y: i32) {
	println!("The value for 'x' is: {}, the value for 'y' is: {}", x, y);
}

fn five() -> i32 {
	5 // don't need to explicitly call 'return'.
}

fn plus_one(x: i32) -> i32 {
	x + 1 // adding ; will raise an error as it converts the expression to a statement which, when returned, evaluate to nothing, '()'.
}