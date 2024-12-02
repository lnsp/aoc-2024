use std::io::{self, stdin};

fn main() -> io::Result<()> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let mut buffer = String::new();
    while stdin().read_line(&mut buffer)? > 0 {
        reports.push(
            buffer
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
        buffer.clear();
    }

    println!("{}", day02::task1(&reports));
    println!("{}", day02::task2(&reports));
    Ok(())
}
