extern crate rand;

use rand::seq::IteratorRandom;
use std::fs;
use std::io::Write;

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line)
    .expect("Error: Could not read a line");

    return line.trim().to_string()
}
pub fn main() {
    println!("\nWelcome to the annual Dundie Awards Ceremony!
    Please type the name of the Office character you'd like to see a quote from.\n");

    loop {
        let input: String = prompt("> ");
        let path: String = "./dundies/".to_owned() + &input;

        if input == "quit" {
            break;
        }
        else {
            let mut rng = rand::thread_rng();

            let files = fs::read_dir(path).expect("Character not found! Please try again.");

            let file = files.choose(&mut rng).unwrap().unwrap();
            let fp = file.path();

            let quote = fs::read_to_string(fp)
            .expect("Something went wrong reading the file.");

            println!("\n{}", quote);
        };
    }
}
