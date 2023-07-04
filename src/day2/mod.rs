#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum HandShape {
    Rock,
    Paper,
    Scissors
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Results {
    Lose,
    Draw,
    Win
}

use std::{fs, collections::HashMap};

fn get_shape_from_char(char: &char) -> HandShape{
    match char {
        'A' => HandShape::Rock,
        'B' => HandShape::Paper,
        'C' => HandShape::Scissors,
        _ => panic!("Invalid shape")
    }
}

fn get_expected_result_from_char(char: &char) -> Results {
    match char {
        'X' => Results::Lose,
        'Y' => Results::Draw,
        'Z' => Results::Win,
        _ => panic!("Invalid result")
    }
}

pub fn run(){    
    let points_by_shape: HashMap<HandShape, u32> = HashMap::from([
        (HandShape::Rock, 1),
        (HandShape::Paper, 2),
        (HandShape::Scissors, 3)
    ]);

    let win_map: HashMap<HandShape, HandShape> = HashMap::from([
        (HandShape::Rock, HandShape::Scissors),
        (HandShape::Paper, HandShape::Rock),
        (HandShape::Scissors, HandShape::Paper)
    ]);

    const POINTS_IF_WIN: u32 = 6;
    const POINTS_IF_DRAW: u32 = 3;

    let mut points: u32 = 0;

    let input = match fs::read_to_string("src/day2/input.txt") {
        Ok(input) => input,
        Err(error) => error.to_string()
    };

    let rounds: Vec<&str> = input.lines().collect();

    for round in rounds.iter() {
        let opponent_char = round.chars().nth(0).unwrap();
        let result_char = round.chars().nth(2).unwrap();

        let opponent_shape = get_shape_from_char(&opponent_char);
        let expected_result = get_expected_result_from_char(&result_char);
        let my_shape = match expected_result {
            Results::Lose => win_map[&opponent_shape],
            Results::Draw => opponent_shape,
            Results::Win => *win_map.keys().into_iter().find(|&shape| win_map[shape] == opponent_shape).unwrap()
        };

        let shape_points = points_by_shape[&my_shape];

        let round_points = match expected_result {
            Results::Lose => 0,
            Results::Draw => POINTS_IF_DRAW,
            Results::Win => POINTS_IF_WIN
        };

        points += round_points + shape_points;
    }

    println!("Day 2: {}", points);
}