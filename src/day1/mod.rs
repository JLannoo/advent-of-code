use std::fs;

pub fn run(){
    let input = match fs::read_to_string("./src/day1/input.txt") {
        Ok(input) => input,
        Err(error) => error.to_string()
    };
    
    let lines_per_elf: Vec<&str> = input.split("\n\n").collect();
    let calories_per_elf: Vec<Vec<&str>> = lines_per_elf.iter().map(|e| e.split("\n").collect()).collect();

    let mut sum_per_elf: Vec<u32> = calories_per_elf.iter().map(|e| 
        e.iter().map(|e| e.parse::<u32>().unwrap_or(0)).sum()
    ).collect();

    println!("The elf with the most calories has {} calories", sum_per_elf.iter().max().unwrap_or(&0));
    
    sum_per_elf.sort();
    sum_per_elf.reverse();
    let top_three_sum: u32 = sum_per_elf[..3].iter().sum();
    
    println!("The top three elves have a total of {} calories", top_three_sum);
}