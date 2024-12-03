use std::io::{self, stdin};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    while stdin().read_line(&mut buffer)? > 0 {};

    println!("{}", aoc2024::day03::task1(&buffer));
    println!("{}", aoc2024::day03::task2(&buffer));

    Ok(())
}
