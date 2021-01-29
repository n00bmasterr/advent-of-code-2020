use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let contents = read_file();
    let split_newline: Vec<&str> = contents.split("\n").collect();
    let mut final_count = 0;
    for lines in split_newline {
        let split_spaces: Vec<&str> = lines.split(" ").collect();
        let split_nums: Vec<&str> = split_spaces[0].split("-").collect();
        let max:u16 = split_nums[1].parse().unwrap();
        let min:u16 = split_nums[0].parse().unwrap();
        let mut character = split_spaces[1].to_string();
        character.pop(); // Remove the last :
        let occurence: u16 = split_spaces[2].matches(&character).count() as u16;
        if occurence >= min && occurence <= max{
            final_count +=1 ;
        }
    }
    println!("The answer is: {}", final_count);
}

pub fn read_file() -> String {
    let mut file =  File::open("password-db.txt").expect("Cannot open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");
    contents
}