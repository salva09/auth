#[macro_use]
extern crate diesel;
extern crate dotenv;

mod hasher;
mod database;

use hasher::hash;

fn main() {
    println!("{}", hash("Salvador"));
}
