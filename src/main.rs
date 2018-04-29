extern crate mlib;
mod que;
use mlib::*;
fn main() {
    let T = readInt32();
    let mut strg: Vec<char> = Vec::new();
    /*readString(&mut strg);
    println!("{:?}", strg);
    for chart in strg.iter() {
        print!("{} ",chart);
    }*/
    que::strings::solve::first_occurence();
}
