fn main() {
	let my_string = String::from("hello world");

    // first_word works on slices of `String`s
	let word = first_word(&my_string[..]);
	let word_two = second_word(&my_string[..]);
	println!("Word: '{}' || First word (&my_string[..]): {} || Second word (&my_string[..]): {}", my_string, word, word_two);

	
    let my_string_literal = "hello world";

    // first_word works on slices of string literals
	let word = first_word(&my_string_literal[..]);
	let word_two = second_word(&my_string_literal[..]);
	println!("Word: '{}' || First word (&my_string_literal[..]): {} || Second word (&my_string_literal[..]): {}", my_string, word, word_two);


    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
	let word = first_word(my_string_literal);
	let word_two = second_word(my_string_literal);
	println!("Word: '{}' || First word (my_string_literal: {} || Second word (my_string_literal): {}", my_string, word, word_two);


    let mut s = String::from("hello world");

	{
		// Resolve reference error to 's' by creating a separate scope for word.
		let word = first_word(&s[..]); // word will get the value 5
		println!("Word: '{}' || First word end index: {}", s, word);
	}
	
	s.clear(); // this empties the String, making it equal to ""
	println!("Word: '{}'", s);

    // word still has the value 5 here, but there's no more string that
	// we could meaningfully use the value 5 with. word is now totally invalid!
	

	// String slices:
	let s = String::from("hello world");
	println!("s: {}", s);

    let hello = &s[0..5];
	let world = &s[6..11];
	
	println!("&s[0..5]: '{}', &s[6..11]: '{}'", hello, world);

	let s = String::from("hello");

	let slice = &s[0..2];
	println!("&s[0..2]: {}", slice);
	let slice = &s[..2];
	println!("&s[..2]: {}", slice);

	let len = s.len();

	let slice = &s[3..len];
	println!("&s[3..len]: {}", slice);
	let slice = &s[3..];
	println!("&s[3..]: {}", slice);

	let slice = &s[0..len];
	println!("&s[0..len]: {}", slice);
	let slice = &s[..];
	println!("&s[..]: {}", slice);


	let a = [1, 2, 3, 4, 5];
	let slice = &a[1..3];

}


fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[..i];
		}
	}

	&s[..]
}

fn second_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	let mut count = 0;
	let mut start_index = 0;
	let mut end_index = 0;

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			if count == 0 {
				start_index = i;
			} else {
				end_index = i;
			}
			count = count + 1;
		}
		if count == 2 {
			return &s[start_index..end_index];
		}
	}
	if count == 0 {
		return "";
	} else {
		return &s[start_index..];
	}
}