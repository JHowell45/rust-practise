fn main() {
	let x = 2.0; // f64
	println!("x: {}", x);
	let y: f32 = 3.0; // f32
	println!("y: {}", y);

	let tup: (i32, f64, u8) = (500, 6.4, 1);

	let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
