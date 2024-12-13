use std::{
    error::Error,
    io::{Read, stdin},
};

use aoc2024::day12;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let plots: Vec<Vec<char>> = buffer
        .split_whitespace()
        .map(|row| row.chars().collect())
        .collect();

    println!("{}", day12::task1(&plots));
    println!("{}", day12::task2(&plots));

    Ok(())
}
