//print first occurrence
use std::io;
use mlib::*;

pub fn first_occurence(){
    let mut input = Vec::new();
    readString(&mut input);
    input.dedup();

    let mut temp: Vec<char> = Vec::new();

    for char in input.iter() {
        if check(&temp, *char){continue;}
        else { temp.push(*char); }
    }
    println!("{:?}" ,temp);

}

fn check(temp: &Vec<char>, cha: char) ->bool{
    let mut flag = false;
    for ch in temp.iter() {
        if *ch == cha { flag = true ; break;}
        else { flag = false }
    }
    flag
}