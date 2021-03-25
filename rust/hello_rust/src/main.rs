use std::io::{self, Write};

fn main() {
    let message = String::from("World");
    say_hello(message);
    let question = String::from("What's your name? - ");
    print!("{}", question);
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Something went wrong");
    say_hello(name.trim().to_string());
}

fn say_hello(the_word: String) {
    println!("Hello {}!", the_word);
}
