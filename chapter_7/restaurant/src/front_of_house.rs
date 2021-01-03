pub mod hosting { // a module can have sub-modules
	pub fn add_to_waitlist() {}
	#[allow(dead_code)]
	fn seat_at_table() {}
}
pub mod serving {
	#[allow(dead_code)]
	fn take_order() {}
	#[allow(dead_code)]
	pub fn serve_order() {}
	#[allow(dead_code)]
	fn take_payment() {}
}