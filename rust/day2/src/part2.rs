use crate::part1;

pub fn run() {
    let contents = part1::read_file();
    let split_newline: Vec<&str> = contents.split("\n").collect();
    let mut final_count = 0;
    for lines in split_newline {
        let split_spaces: Vec<&str> = lines.split(" ").collect();
        let split_nums: Vec<&str> = split_spaces[0].split("-").collect();
        let first_index = split_nums[1].parse().unwrap() - 1;
        let second_index = split_nums[0].parse().unwrap() - 1;
        let mut character = split_spaces[1].to_string().chars();
        character.pop(); // Remove the last :
        if Some(character) == split_spaces[2].chars().nth(0) {
            println!("{}",character);
        }
    }
    println!("The answer is: {}", final_count);

}
