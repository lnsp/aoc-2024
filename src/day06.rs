use std::collections::HashSet;

pub type MapRow = Vec<MapEntry>;
pub type Map = Vec<MapRow>;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MapEntry {
    Air,
    Wall,
    Guard,
}

pub fn entry_from_char(ch: char) -> Option<MapEntry> {
    match ch {
        '#' => Some(MapEntry::Wall),
        '.' => Some(MapEntry::Air),
        '^' => Some(MapEntry::Guard),
        v => panic!("unexpected symbol {}", v),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, guard: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (guard.0 - 1, guard.1),
            Direction::Down => (guard.0 + 1, guard.1),
            Direction::Left => (guard.0, guard.1 - 1),
            Direction::Right => (guard.0, guard.1 + 1),
        }
    }
    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn within_bounds(guard: (i32, i32), bounds: (usize, usize)) -> bool {
    return guard.0 >= 0 && guard.1 >= 0 && guard.0 < bounds.0 as i32 && guard.1 < bounds.1 as i32;
}

fn map_entry_at(map: &Map, guard: (i32, i32)) -> Option<MapEntry> {
    let bounds = (map.len(), map[0].len());
    if !within_bounds(guard, bounds) {
        None
    } else {
        Some(map[guard.0 as usize][guard.1 as usize])
    }
}

fn find_visited_positions(map: &Map, mut guard: (i32, i32)) -> Vec<(i32, i32)> {
    let bounds = (map.len(), map[0].len());
    let mut visited: Vec<Vec<bool>> = vec![vec![false; bounds.1]; bounds.0];
    let mut dir = Direction::Up;

    while within_bounds(guard, bounds) {
        visited[guard.0 as usize][guard.1 as usize] = true;

        let next_guard = dir.next(guard);
        if let Some(MapEntry::Wall) = map_entry_at(&map, next_guard) {
            dir = dir.turn();
        } else {
            guard = next_guard;
        }
    }

    visited
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .filter(|&(_, f)| f)
                .map(move |(j, _)| (i as i32, j as i32))
        })
        .collect()
}

pub fn task1(map: &Map, guard: (i32, i32)) -> usize {
    find_visited_positions(map, guard).len()
}

fn has_loop(map: &Map, mut guard: (i32, i32)) -> bool {
    let bounds = (map.len(), map[0].len());
    let mut visited: Vec<Vec<HashSet<Direction>>> = vec![vec![HashSet::new(); bounds.1]; bounds.0];
    let mut dir = Direction::Up;

    while within_bounds(guard, bounds) {
        if !visited[guard.0 as usize][guard.1 as usize].insert(dir) {
            return true;
        }

        let next_guard = dir.next(guard);
        if let Some(MapEntry::Wall) = map_entry_at(&map, next_guard) {
            dir = dir.turn();
        } else {
            guard = next_guard;
        }
    }
    false
}

pub fn task2(map: &Map, guard: (i32, i32)) -> usize {
    let mut map_copy = map.clone();
    find_visited_positions(map, guard).iter().cloned().filter(|&(i, j)| {
        if (i, j) == guard {
            return false
        }
        map_copy[i as usize][j as usize] = MapEntry::Wall;
        let found_loop = has_loop(&map_copy, guard);
        map_copy[i as usize][j as usize] = MapEntry::Air;
        found_loop
    }).count()
}
