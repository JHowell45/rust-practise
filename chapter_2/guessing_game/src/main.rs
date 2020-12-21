use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0..101);
    let mut tries = 0;
    println!("Current no. of tries: {}", tries);


    loop {
        println!("Please input your guess: ");
        
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        tries += 1;
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Current no. of tries: {}", tries);
                println!("You win!");
                break;
            },
        }
        println!("Current no. of tries: {}", tries);
    }

}
