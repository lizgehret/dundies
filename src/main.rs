extern crate rand;

use rand::seq::IteratorRandom;
use std::fs;
pub fn main() {

    let mut rng = rand::thread_rng();

    let files = fs::read_dir("./dundies").unwrap();
    let file = files.choose(&mut rng).unwrap().unwrap();
    let fp = file.path();
    println!("{:?}", fp);
    let quote = fs::read_to_string(fp)
    .expect("Something went wrong reading the file.");

    let q = quote.trim_end();

    println!("{:#?}", q);
}
