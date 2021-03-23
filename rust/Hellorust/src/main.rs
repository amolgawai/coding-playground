use std::io;

fn main() {
    let message = String::from("Hello World!");
    println!("{}", message);
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Something went wrong");
    // println!("{}", name);
    say_hello(name)
}

fn say_hello(the_word: String) {
    println!("Hello {}", the_word);
}
