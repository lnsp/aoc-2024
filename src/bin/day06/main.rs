use std::io::{self, stdin};

use aoc2024::day06::{self, MapEntry};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut map = day06::Map::new();
    let mut guard: (i32, i32) = (0, 0);

    while stdin().read_line(&mut buffer)? > 0 {
        let mut row: Vec<MapEntry> = buffer
            .trim()
            .chars()
            .map(|c| day06::entry_from_char(c).unwrap())
            .collect();
        match row.iter().position(|&v| v == day06::MapEntry::Guard) {
            Some(guard_col) => {
                guard = (map.len() as i32, guard_col as i32);
                row[guard_col] = day06::MapEntry::Air;
            }
            _ => (),
        }
        map.push(row);
        buffer.clear();
    }

    println!("{}", day06::task1(&map, guard));
    println!("{}", day06::task2(&map, guard));

    Ok(())
}
