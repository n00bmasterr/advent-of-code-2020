use crate::part1;

pub fn run() {
    let contents = part1::read_file();
    let split_newline: Vec<&str> = contents.split("\n").collect();
    let mut final_count = 0;
    for lines in split_newline {
        let split_spaces: Vec<&str> = lines.split(" ").collect();
        let split_nums: Vec<&str> = split_spaces[0].split("-").collect();
        let first_index: u16 = split_nums[1].parse::<u16>().unwrap() - 1;
        let second_index: u16 = split_nums[0].parse::<u16>().unwrap() - 1;
        let mut character = split_spaces[1].to_string();
        character.pop(); // Remove the last :
        let test = character.chars().nth(0).unwrap();
        if split_spaces[2].chars().nth(first_index as usize) == Some(test) && split_spaces[2].chars().nth(second_index as usize) != Some(test) {
             final_count += 1;
        } else if split_spaces[2].chars().nth(first_index as usize) != Some(test) && split_spaces[2].chars().nth(second_index as usize) == Some(test) {
            final_count += 1;
        }
    }
    println!("The answer is: {}", final_count);

}
