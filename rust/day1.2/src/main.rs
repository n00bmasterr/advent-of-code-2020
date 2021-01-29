use std::fs::File;
use std::io::prelude::*;


fn main(){
    println!("Opening file");
    // Open the file 
    let mut file =  File::open("report.txt").expect("Cannot open file");
    // Store the data in the file in this variable   
    let mut contents = String::new();
    // Read the file and if there is an error then just output the error message.
    file.read_to_string( /* Because it needs to be mutable */ &mut contents).expect("Can't read file");
    find_num(contents);
}

fn find_num( content : String ) -> u8 {
    // Create a vector that we will store these values inside
    let mut reports: Vec<u64> = vec![];

    let split_string = content.split("\n");
    for line in split_string{
        reports.push(line.parse::<u64>().unwrap());
    }
    for i in 0..reports.len(){
        for j in 0..reports.len(){
            for k in 0.. reports.len() {
                if i != j && j != k && reports[i] + reports[j] + reports[k] == 2020 {
                    let mut answer = reports[i] * reports[j] * reports[k];
                    println!("Found the answer: {}", answer);
                    return 0 as u8
                }
            } 
        }
    }
    return 1 as u8
}