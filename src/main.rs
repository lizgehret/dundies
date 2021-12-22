extern crate rand;

use rand::seq::IteratorRandom;
use std::fs;
use std::env;
pub fn main() {
    // TODO: print welcome message before prompting user input
    // println!("Welcome to the annual Dundie Awards Ceremony!
    // Please type the name of the Office character you'd like to see a quote from.\n");

    let args: Vec<String> = env::args().collect();
    let char = &args[1];

    let path = "./dundies/".to_owned() + char;

    let mut rng = rand::thread_rng();

    let files = fs::read_dir(path).unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let fp = file.path();

    let quote = fs::read_to_string(fp)
    .expect("Something went wrong reading the file.");

    println!("\n{}", quote);
}
