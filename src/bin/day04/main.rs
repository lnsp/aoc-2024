use std::io::{self, stdin};

use aoc2024::day04;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut rows = Vec::new();
    while stdin().read_line(&mut buffer)? > 0 {
        rows.push(buffer.trim().as_bytes().to_vec());
        buffer.clear();
    }

    println!("{}", day04::task1(&rows));
    println!("{}", day04::task2(&rows));

    Ok(())
}
