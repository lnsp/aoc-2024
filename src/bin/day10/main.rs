use std::{
    error::Error,
    io::{Read, stdin},
};

use aoc2024::day10;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let trailmap: Vec<Vec<u8>> = buffer
        .trim()
        .split_whitespace()
        .map(|row| row.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect();

    println!("{}", day10::task1(&trailmap));
    println!("{}", day10::task2(&trailmap));

    Ok(())
}
