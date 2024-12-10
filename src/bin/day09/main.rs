use std::{
    error::Error,
    io::{Read, stdin},
};

use aoc2024::day09;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer)?;

    let filesystem = buffer
        .trim()
        .as_bytes()
        .iter()
        .enumerate()
        .fold(
            (Vec::<(i32, usize)>::new(), false),
            |(mut memory, free_space), (file_index, c)| {
                let n = c - '0' as u8;
                if free_space {
                    memory.push((-1, n as usize));
                } else {
                    memory.push(((file_index / 2) as i32, n as usize));
                }
                (memory, !free_space)
            },
        )
        .0;

    println!("{}", day09::task1_better(filesystem.clone()));
    println!("{}", day09::task2(filesystem.clone()));

    Ok(())
}
