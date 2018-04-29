use std::io;

pub fn prt() {
    println!("Welcome to the my world");
}


//this function will take mutable vec as argument and return
//a vector of chars.
pub fn readString(strg: &mut Vec<char>) {
    let mut ip_srt = String::new();

    io::stdin().read_line(&mut ip_srt).expect("Error While reading");
    ip_srt.trim().to_string();
    for char in ip_srt.chars() {
        strg.push(char);
    }
}

pub fn readInt32() -> i32 {
    let mut ip_txt = String::new();
    io::stdin().read_line(&mut ip_txt).expect("error while reading");
    let trimed = ip_txt.trim();
    trimed.parse::<i32>().unwrap()
}

pub fn readInt16() -> i16 {
    let mut ip_txt = String::new();
    io::stdin().read_line(&mut ip_txt).expect("error while reading");
    let trimed = ip_txt.trim();
    trimed.parse::<i16>().unwrap()
}

pub fn readInt8() -> i8 {
    let mut ip_txt = String::new();
    io::stdin().read_line(&mut ip_txt).expect("error while reading");
    let trimed = ip_txt.trim();
    trimed.parse::<i8>().unwrap()
}