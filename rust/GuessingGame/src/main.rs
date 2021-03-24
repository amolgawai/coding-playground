// Guessing Game
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

	let secret_num = rand::thread_rng().gen_range(1..101);

	loop {
		println!("Please enter your guess");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("Couldn't read the guess");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please enter a number");
				continue;
			},
		};
		match guess.cmp(&secret_num) {
			Ordering::Less => println!("Too Low"),
			Ordering::Greater => println!("Too High"),
			Ordering::Equal => {
				println!("Bingo!!, you win");
				break;
			}
		}
	}
}
