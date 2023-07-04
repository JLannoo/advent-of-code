use std::{fs, ascii::AsciiExt};

/*
- Lowercase or uppercase => type of item
- Each line => list of items for each rucksack
- First half => first compartment
- Second half => second compartment
- Lowercase items => priority 1 - 26
- Uppercase items => priority 27 - 52

 intermediate step => Item that appears in both compartments
 final question => sum of the priorities
*/

/*
Part 2
- Each 3 lines => group
- Each group => have an item that appears in all 3 rucksacks

 intermediate step => Item that appears in all 3 rucksacks
 final question => sum of the priorities
*/

fn get_priority_from_char(char: &char) -> u64 {
    match char {
        char if char.is_lowercase() => char.to_ascii_lowercase() as u64 - 96,
        char if char.is_uppercase() => char.to_ascii_uppercase() as u64 - 38,
        _ => 0
    }
}

pub fn run(){
    let input = fs::read_to_string("./src/day3/input.txt").unwrap();
    let rucksacks: Vec<&str> = input.lines().collect();

    let mut repeated_items: Vec<char> = Vec::new();
    let mut priority_sum: u64 = 0;

    for chunk in rucksacks.chunks(3) {
        let (r1, r2, r3) = (chunk[0], chunk[1], chunk[2]);

        for (item) in r1.chars() {
            if r2.contains(item) && r3.contains(item) {
                repeated_items.push(item);
                break;
            }
        }
    }

    println!("The repeated items are: {:?}", repeated_items);

    for item in repeated_items.iter() {
        let priority = get_priority_from_char(item);

        priority_sum += priority;
    }

    println!("The sum of the priorities is: {}", priority_sum);
}