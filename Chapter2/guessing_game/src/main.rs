use std::io;
use rand::Rng;
use std::cmp::Ordering;

/*
**	Wassup
*/

fn main() {
    println!("Guess the number!");

	let mut min: u32 = 1;
	let mut max: u32 = 101;
    let secret_number = rand::thread_rng().gen_range(min, max);

    // println!("The secret number is: {}", secret_number);


	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

	    io::stdin()
	        .read_line(&mut guess)
	        .expect("Failed to read line");

	    println!("You guessed: {}", guess);
		let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
		let guess: u32 = ((max - min) / 2) + min;

		println!("The number is: {}", guess);

	    match guess.cmp(&secret_number) {
	        Ordering::Less => {
				println!("Too small!");
				min = guess;
			},
	        Ordering::Greater =>
			{
				println!("Too big!");
				max = guess;
			},
	        Ordering::Equal => {
				println!("You win!");
				break;
			}
	    }
	}
}
