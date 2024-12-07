use std::io::{self, stdin};

use aoc2024::day07::{self};

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let mut equations = Vec::<day07::Equation>::new();
    while stdin().read_line(&mut buffer)? > 0 {
        let parts: Vec<&str> = buffer.split(":").collect();
        equations.push(day07::Equation {
            sum: parts[0].parse().unwrap(),
            values: parts[1]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        });
        buffer.clear();
    }

    println!("{}", day07::task1(&equations));
    println!("{}", day07::task2(&equations));

    Ok(())
}
