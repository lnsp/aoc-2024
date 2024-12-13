use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, LinkedList},
};

use crate::day10;
use strum::IntoEnumIterator;

pub fn task1(plots: &Vec<Vec<char>>) -> usize {
    let bounds = (plots.len(), plots[0].len());
    let mut visited = vec![vec![false; bounds.1]; bounds.0];
    let mut total = 0;

    while let Some((si, sj)) = visited
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, p)| !p)
                .map(move |(j, _)| (i, j))
        })
        .next()
    {
        let (mut area, mut perimeter) = (0, 0);
        let mut stack = LinkedList::<(usize, usize)>::new();
        stack.push_back((si, sj));

        while let Some((ni, nj)) = stack.pop_back() {
            if visited[ni][nj] {
                continue;
            }
            visited[ni][nj] = true;
            area += 1;
            perimeter += 4;

            day10::Direction::iter()
                .flat_map(|dir| dir.follow((ni, nj), bounds))
                .filter(|&(i, j)| plots[i][j] == plots[si][sj])
                .for_each(|(i, j)| {
                    perimeter -= 1;
                    stack.push_back((i, j));
                });
        }

        total += area * perimeter;
    }
    total
}

fn face_of(direction: day10::Direction, (i, j): (usize, usize)) -> (day10::Direction, usize) {
    match direction {
        day10::Direction::Up | day10::Direction::Down => (direction, i),
        day10::Direction::Left | day10::Direction::Right => (direction, j),
    }
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    return (max(a.0, b.0) - min(a.0, b.0)) + (max(a.1, b.1) - min(a.1, b.1));
}

pub fn task2(plots: &Vec<Vec<char>>) -> usize {
    let bounds = (plots.len(), plots[0].len());
    let mut visited = vec![vec![false; bounds.1]; bounds.0];
    let mut total = 0;

    while let Some((si, sj)) = visited
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, p)| !p)
                .map(move |(j, _)| (i, j))
        })
        .next()
    {
        let mut area = 0;
        let mut stack = LinkedList::<(usize, usize)>::new();
        let mut faces = HashMap::<(day10::Direction, usize), Vec<(usize, usize)>>::new();
        stack.push_back((si, sj));

        while let Some((ni, nj)) = stack.pop_back() {
            if visited[ni][nj] {
                continue;
            }
            visited[ni][nj] = true;
            area += 1;

            for direction in day10::Direction::iter() {
                if let Some((i, j)) = direction
                    .follow((ni, nj), bounds)
                    .filter(|&(i, j)| plots[i][j] == plots[si][sj])
                {
                    stack.push_back((i, j));
                } else {
                    faces
                        .entry(face_of(direction, (ni, nj)))
                        .or_insert(Vec::new())
                        .push((ni, nj));
                }
            }
        }

        // Count distinct face sets
        let mut face_sets = 0;
        for (_, points) in faces {
            let mut point_set = HashSet::<(usize, usize)>::from_iter(points.iter().cloned());
            while let Some(&initial) = point_set.iter().next() {
                point_set.remove(&initial);
                face_sets += 1;
                // Check for any matches
                let mut next_up = LinkedList::<(usize, usize)>::new();
                next_up.push_back(initial);
                while let Some(current) = next_up.pop_back() {
                    point_set.remove(&current);
                    point_set
                        .iter()
                        .filter(|&candidate| manhattan(*candidate, current) == 1)
                        .for_each(|&candidate| next_up.push_back(candidate));
                }
            }
        }

        total += area * face_sets;
    }
    total
}
