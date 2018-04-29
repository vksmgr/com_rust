extern crate mlib;

mod que;

use mlib::*;

fn main() {
    let T = readInt32();
    let mut strg: Vec<char> = Vec::new();

    for _ in 0..T {
        que::strings::solve::first_occurence();
    }
}
