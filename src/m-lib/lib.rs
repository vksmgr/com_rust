use std::io;
pub fn prt(){
    println!("Welcome to the my world");
}


//this function will take mutable vec as argument and return
//a vector of chars.
pub fn readString(strg: &mut Vec<char>){
    let mut ip_srt = String::new();

    io::stdin().read_line(&mut ip_srt).expect("Error While reading");
    ip_srt.trim().to_string();
    for char in ip_srt.chars() {
        strg.push(char);
    }
}