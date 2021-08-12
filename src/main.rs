extern crate rustc_serialize;
use rustc_serialize::json::{self,ToJson, Json};

struct Dvd {
    name: String,
    year: u16,
    cast: String,
    length: u16,
}



fn main() {
    println!("Hello, world!");
}
