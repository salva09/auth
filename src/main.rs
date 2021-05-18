mod hasher;
mod database;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use hasher::hash;

fn main() {
    println!("{}", hash("Salvador"));
}
