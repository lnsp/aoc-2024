use std::{
    error::Error,
    io::{Read, stdin},
};

use aoc2024::day11;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let stones: Vec<usize> = buffer
        .trim()
        .split_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .collect();

        println!("{}", day11::task1(&stones));
        println!("{}", day11::task2(&stones));

    Ok(())
}
