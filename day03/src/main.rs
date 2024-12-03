use std::io::{self, stdin};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    while stdin().read_line(&mut buffer)? > 0 {};

    println!("{}", day03::task1(&buffer));
    println!("{}", day03::task2(&buffer));

    Ok(())
}
