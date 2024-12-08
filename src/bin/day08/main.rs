use std::{collections::HashMap, error::Error, io::stdin};

use aoc2024::day08;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut bounds = (0, 0);
    let mut antennas = HashMap::<char, Vec<(i32, i32)>>::new();

    while stdin().read_line(&mut buffer)? > 0 {
        buffer
            .trim()
            .chars()
            .enumerate()
            .filter(|&(_, c)| c != '.')
            .for_each(|(col, c)| antennas.entry(c).or_insert(Vec::new()).push((bounds.0, col as i32)));

        bounds.0 += 1;
        bounds.1 = buffer.trim().chars().count() as i32;
        buffer.clear();
    }

    println!("{}", day08::task1(&antennas, bounds));
    println!("{}", day08::task2(&antennas, bounds));

    Ok(())
}
