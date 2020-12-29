fn main() {
    let width1 = 30;
	let height1 = 50;
	
	println!("The area of the rectangle is {} square pixels.", area(width1, height1));

	let dimensions1 = (width1, height1);
	println!("The area of the rectangle is {} square pixels.", refactored_area(dimensions1));

	let rect1 = Rect {width: width1, height: height1};
	println!("The area of the rectangle is {} square pixels.", rect_area(&rect1));

	println!("Rectangle: {:?}", rect1);
	println!("Rectangle: {:#?}", rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn refactored_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area(r: &Rect) -> u32 {
	r.width * r.height
}

#[derive(Debug)]
struct Rect {
	width: u32,
	height: u32,
}