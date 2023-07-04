/*
    Crates => are stacked in stacks
    crates only move one at a time

    final question: which crate is at the bottom of each stack
*/

use std::{fs};

#[derive (Debug)]
struct Stack {
    crates: Vec<char>
}

#[derive (Debug)]
struct Dock {
    stacks: Vec<Stack>
}

impl Dock {
    fn move_crates(&mut self, quantity: u32, from: u32, to: u32) {
        let mut crates: Vec<char> = Vec::new();

        let crates = &mut self.stacks[from as usize - 1].crates;
        let picked_crates = crates.split_off(crates.len() - quantity as usize);

        for crate_ in picked_crates {
            self.stacks[to as usize - 1].crates.push(crate_);
        }
    }

    fn parse_stacks(stack_string: &str) -> Vec<Stack> {
        let mut stacks: Vec<Stack> = Vec::new();

        let numbers = stack_string.lines().last().unwrap().chars().collect::<Vec<char>>();

        for number in numbers.iter().filter(|x| x != &&' ') {
            let index = numbers.iter().position(|x| x == number).unwrap();

            let mut stack = Stack {
                crates: Vec::new()
            };

            let lines_except_last = stack_string.lines().rev().skip(1);

            for line in lines_except_last {
                let crate_char = line.chars().nth(index).unwrap();
                if(crate_char != ' ') {
                    stack.crates.push(crate_char);
                }
            }

            stacks.push(stack);
        }

        stacks
    }

    fn get_top_crates(&self) -> Vec<char> {
        self.stacks.iter().map(|stack| stack.crates.last().unwrap().clone()).collect()
    }
}

impl std::fmt::Display for Dock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for (i, stack) in self.stacks.iter().enumerate() {
            string.push_str(
                &format!(
                "{} - {:?} - length: {}\n", 
                i+1, stack, stack.crates.len()
            ));
        }

        write!(f, "{}", string)
    }
}

pub fn run(){
    let input = fs::read_to_string("./src/day5/input.txt").unwrap();

    let (stacks , instructions) = split_to_tuple("\n\n", &input);
    let stacks = Dock::parse_stacks(stacks);

    let mut dock = Dock { stacks };

    for instruction in instructions.lines() {
        println!("{}", dock);
        println!("{}", instruction);

        let (quantity, from, to) = parse_instruction(instruction);        
        dock.move_crates(quantity, from, to);
    }

    let top_crates: String = dock.get_top_crates().iter().collect();

    println!("{}", top_crates);
}



fn split_to_tuple<'a>(separator: &str, str: &'a str) -> (&'a str, &'a str) {
    let pairs: Vec<&str> = str.split(separator).collect();
    (pairs[0], pairs[1])
}

fn parse_instruction(instruction: &str) -> (u32, u32, u32) {
    let splits: Vec<&str> = instruction.split(' ').collect();

    (
        splits[1].parse::<u32>().unwrap(),
        splits[3].parse::<u32>().unwrap(),
        splits[5].parse::<u32>().unwrap(),
    )
}