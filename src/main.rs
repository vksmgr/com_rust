extern crate mlib;

use mlib::*;
fn main() {
    println!("Hello, world!");
    prt();
    let mut strg: Vec<char> = Vec::new();
    readString(&mut strg);
    println!("{:?}", strg);
    for chart in strg.iter() {
        print!("{} ",chart);
    }
}
