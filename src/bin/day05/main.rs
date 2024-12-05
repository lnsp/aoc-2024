use std::io::{self, stdin};

use aoc2024::day05;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut orderings = Vec::<(i32, i32)>::new();
    let mut prints = Vec::<Vec<i32>>::new();

    while stdin().read_line(&mut buffer)? > 0 && buffer.trim().len() > 0 {
        orderings.push(
            buffer
                .trim()
                .split("|")
                .map(|v| v.parse::<i32>().unwrap())
                .next_tuple()
                .unwrap(),
        );
        buffer.clear();
    }

    while stdin().read_line(&mut buffer)? > 0 {
        prints.push(
            buffer
                .trim()
                .split(",")
                .map(|v| v.parse::<i32>().unwrap())
                .collect(),
        );
        buffer.clear();
    }
  
    println!("{}", day05::task1(&orderings, &prints));
    println!("{}", day05::task2(&orderings, &prints));

    Ok(())
}
