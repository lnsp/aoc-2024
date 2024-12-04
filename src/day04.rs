use core::str;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn task1(rows: &Vec<Vec<u8>>) -> usize {
    let mut sum = 0;
    for (i, row) in rows.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if rows[i][j] != ('X' as u8) {
                continue;
            }
            sum += Direction::iter()
                .flat_map(|d| d.take(rows, i as i32, j as i32, 4))
                .filter(|v| str::from_utf8(&v).unwrap_or("") == "XMAS")
                .count();
        }
    }
    sum
}

pub fn task2(rows: &Vec<Vec<u8>>) -> usize {
    let options = [
        ([Direction::LeftUp, Direction::RightDown], ["AM", "AS"]),
        ([Direction::LeftUp, Direction::RightDown], ["AS", "AM"]),
        ([Direction::RightUp, Direction::LeftDown], ["AM", "AS"]),
        ([Direction::RightUp, Direction::LeftDown], ["AS", "AM"]),
    ];
    let mut sum = 0;
    for (i, row) in rows.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if rows[i][j] != ('A' as u8) {
                continue;
            }
            if options
                .iter()
                .filter(|(directions, strings)| {
                    directions
                        .iter()
                        .zip(strings.iter())
                        .all(|(direction, string)| direction.take_str(rows, i, j, 2) == *string)
                })
                .count()
                == 2
            {
                sum += 1;
            }
        }
    }
    sum
}

#[derive(EnumIter)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl Direction {
    fn move_from(&self, (i, j): (i32, i32)) -> (i32, i32) {
        match *self {
            Direction::Left => (i, j - 1),
            Direction::Right => (i, j + 1),
            Direction::Down => (i + 1, j),
            Direction::Up => (i - 1, j),
            Direction::LeftDown => (i + 1, j - 1),
            Direction::LeftUp => (i - 1, j - 1),
            Direction::RightDown => (i + 1, j + 1),
            Direction::RightUp => (i - 1, j + 1),
        }
    }
    pub fn take(&self, rows: &Vec<Vec<u8>>, mut i: i32, mut j: i32, n: usize) -> Option<Vec<u8>> {
        let (dim_i, dim_j, mut k) = (rows.len() as i32, rows[0].len() as i32, 0);
        let mut items = Vec::<u8>::new();
        while i >= 0 && j >= 0 && i < dim_i && j < dim_j && k < n {
            items.push(rows[i as usize][j as usize]);
            (i, j) = self.move_from((i, j));
            k += 1;
        }
        if items.len() == n { Some(items) } else { None }
    }

    pub fn take_str(&self, rows: &Vec<Vec<u8>>, i: usize, j: usize, n: usize) -> String {
        let vec = self.take(rows, i as i32, j as i32, n).unwrap_or(Vec::new());
        str::from_utf8(&vec).unwrap().to_string()
    }
}
