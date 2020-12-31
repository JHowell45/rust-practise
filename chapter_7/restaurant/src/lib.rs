

mod front_of_house { // defining a new module
	mod hosting { // a module can have sub-modules
		fn add_to_waiting_list() {}
		fn seat_at_table() {}
	}
	mod serving {
		fn take_order() {}
		fn serve_order() {}
		fn take_payment() {}
	}
}