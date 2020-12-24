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

	let a = [1, 2, 3, 4, 5];

    let first = a[0];
	let second = a[1];
	
    println!("The value of first is: {}", first);
	println!("The value of second is: {}", second);
	
	let x: (i32, f64, u8) = (500, 6.4, 1);

    let a = x.0;
    let b = x.1;
	let c = x.2;
	
	println!("The value of a is: {}", a);
	println!("The value of b is: {}", b);
	println!("The value of c is: {}", c);
}
