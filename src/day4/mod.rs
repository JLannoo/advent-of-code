use std::fs;

/*
    section assignments by pairs
    2-4,6-8 (example)

    intermediate step => find pair such that one fully contains the other
    final question => total amount of contained pairs
*/

/*
    PART 2
    The same but not fully contain, but overlap
*/
struct Range {
    start: u32,
    end: u32
}

impl Range {
    fn numbers_in_range(&self) -> Vec<u32> {
        (self.start..self.end+1).collect()
    }
}

fn split_to_tuple(separator: char, str: &str) -> (&str, &str) {
    let pairs: Vec<&str> = str.split(separator).collect();
    (pairs[0], pairs[1])
}

fn get_range(pair: &str) -> Range {
    let (first, second) = split_to_tuple('-', pair);
    
    Range {
        start: first.parse::<u32>().unwrap(),
        end: second.parse::<u32>().unwrap()
    }
}

pub fn run(){
    let input = fs::read_to_string("./src/day4/input.txt").unwrap();
    let pairs: Vec<&str> = input.lines().collect();

    let mut overlapping = 0;

    for pair in pairs {
        let (first, second) = split_to_tuple(',', pair);
        
        let first_range = get_range(first);
        let second_range = get_range(second);

        let first_numbers = first_range.numbers_in_range();
        let second_numbers = second_range.numbers_in_range();

        for number in first_numbers {
            if second_numbers.contains(&number) {
                overlapping += 1;
                break;
            }
        }

    }

    println!("The number of overlaps is: {}", overlapping)
}