use std::{
    error::Error,
    io::stdin,
};

use aoc2024::day13;
use itertools::Itertools;

use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let button_regex = Regex::new(r"Button [A-Z]: X\+([0-9]+), Y\+([0-9]+)")?;
    let prize_regex = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)")?;

    let mut games = Vec::<(Vec<(usize, usize)>, (usize, usize))>::new();
    let mut buffer = String::new();

    let mut current_game = Vec::<(usize, usize)>::new();
    while stdin().read_line(&mut buffer)? > 0 {
        if buffer.trim().is_empty() {
            current_game = Vec::<(usize, usize)>::new();
            continue;
        }
        if buffer.trim().starts_with("Button") {
            if let Some(captures) = button_regex.captures(&buffer) {
                current_game.push(
                    captures
                        .iter()
                        .flat_map(|m| m)
                        .skip(1)
                        .map(|m| m.as_str().parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                );
            }
        } else if buffer.trim().starts_with("Prize") {
            if let Some(captures) = prize_regex.captures(&buffer) {
                let (prize_x, prize_y) = captures
                    .iter()
                    .flat_map(|m| m)
                    .skip(1)
                    .map(|m| m.as_str().parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                games.push((current_game.clone(), (prize_x, prize_y)));
            }
        }
        buffer.clear();
    }

    println!("{}", day13::task1(&games));
    println!("{}", day13::task2(&games));

    Ok(())
}
