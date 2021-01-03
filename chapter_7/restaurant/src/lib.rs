mod front_of_house;

mod back_of_house {
	pub enum Appetizer {
		Soup,
		Salad,
	}
	pub struct Breakfast {
		pub toast: String, // only the toast can be accessed publicaly.
		#[allow(dead_code)]
        seasonal_fruit: String, // a private attr. 
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
	}
	#[allow(dead_code)]
	fn cook_order() {}
	#[allow(dead_code)]
	fn fix_incorrect_order() {
		cook_order();
		super::front_of_house::serving::serve_order();
	}
}

// use crate::front_of_house::hosting;
// use self::front_of_house::hosting; // relative path

// re-exporting
pub use crate::front_of_house::hosting; // by making it public it makes this path accessible to external code.

pub fn eat_at_restaurant() {
	// Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
	// front_of_house::hosting::add_to_waitlist();
	
	// ##################################################################

	let mut meal = back_of_house::Breakfast::summer("Rye");
	meal.toast = String::from("Wheat");
	println!("I'd like {} toast please", meal.toast);

	// The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
	// meal.seasonal_fruit = String::from("blueberries");
	
	// ##################################################################

	let _order1 = back_of_house::Appetizer::Soup;
	let _order2 = back_of_house::Appetizer::Salad;

	// ##################################################################

	// 7.4
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
	hosting::add_to_waitlist();
}

// How to bring two items with the same name into the scope.
use std::fmt;
use std::io;
// an alt:
use std::fmt::Result;
use std::io::Result as IoResult;

#[allow(dead_code)]
fn function1() -> fmt::Result {
    Ok(())
}

#[allow(dead_code)]
fn function2() -> io::Result<()> {
    Ok(())
}

// the alt:
#[allow(dead_code)]
fn function1_alt() -> Result {
    Ok(())
}

#[allow(dead_code)]
fn function2_alt() -> IoResult<()> {
    Ok(())
}