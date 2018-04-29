//print first occurrence
use std::io;
use mlib::*;


//Problem 1
pub fn first_occurence(){
    let mut input = Vec::new();
    readString(&mut input);
    input.dedup();

    let mut temp: Vec<char> = Vec::new();

    for char in input.iter() {
        if check(&temp, *char){continue;}
        else { temp.push(*char); }
    }
    for cha in temp.iter() {
        print!("{}",cha);
    }

}

fn check(temp: &Vec<char>, cha: char) ->bool{
    let mut flag = false;
    for ch in temp.iter() {
        if *ch == cha { flag = true ; break;}
        else { flag = false }
    }
    flag
}

//Problem 2 : Jumble Letter

pub fn jumble_letter(){
    let mut input = Vec::new();
    readString(&mut input);

}