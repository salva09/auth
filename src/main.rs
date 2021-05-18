mod hasher;
mod database;

use hasher::hash;

fn main() {
    println!("{}", hash("Salvador"));
}
