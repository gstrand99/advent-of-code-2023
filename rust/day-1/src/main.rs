use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let mut _sum_of: u32 = 0;

    for contents in fs::read_to_string(file_path).unwrap().lines() {
        let nums: Vec<u32> = contents
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let first = nums[0].to_string();
        let last = nums[nums.len() - 1].to_string();
        let answer: u32 = [first, last].join("").parse::<u32>().unwrap();
        _sum_of += answer;
    }
    println!("The answer is {:?}", _sum_of)
}
