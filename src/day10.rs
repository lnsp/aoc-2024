use std::collections::{HashMap, LinkedList};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn follow(&self, current: (usize, usize), bounds: (usize, usize)) -> Option<(usize, usize)> {
        if match self {
            Direction::Up => current.0 > 0,
            Direction::Down => current.0 < bounds.0 - 1,
            Direction::Left => current.1 > 0,
            Direction::Right => current.1 < bounds.1 - 1,
        } {
            return Some(match self {
                Direction::Up => (current.0 - 1, current.1),
                Direction::Down => (current.0 + 1, current.1),
                Direction::Left => (current.0, current.1 - 1),
                Direction::Right => (current.0, current.1 + 1),
            });
        }
        None
    }
}

pub fn score_trails(
    trailmap: &Vec<Vec<u8>>,
    score_fn: fn(usize) -> usize,
) -> usize {
    let trailbounds = (trailmap.len(), trailmap[0].len());
    trailmap
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, c)| *c == 0)
                .map(move |(j, _)| (i, j))
        })
        .map(|trailhead| {
            let mut visited = HashMap::<(usize, usize), usize>::new();
            let mut stack = LinkedList::<(usize, usize)>::new();
            stack.push_back(trailhead);
            while let Some(current) = stack.pop_back() {
                *visited.entry(current).or_insert(0) += 1;
                [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ]
                .iter()
                .flat_map(|dir| dir.follow(current, trailbounds))
                .filter(|next| trailmap[next.0][next.1] == trailmap[current.0][current.1] + 1)
                .for_each(|next| stack.push_back(next))
            }
        visited
            .iter()
            .filter(|&((i, j), _)| trailmap[*i][*j] == 9)
            .map(|(_, &c)| score_fn(c))
            .sum::<usize>()
        })
        .sum()
}

pub fn task1(trailmap: &Vec<Vec<u8>>) -> usize {
    score_trails(trailmap, |_| 1)
}

pub fn task2(trailmap: &Vec<Vec<u8>>) -> usize {
    score_trails(trailmap, |unique_paths| unique_paths)
}
