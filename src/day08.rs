use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn within_bounds(point: (i32, i32), bounds: (i32, i32)) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < bounds.0 && point.1 < bounds.1
}

fn reflect_antennas(left: &(i32, i32), right: &(i32, i32)) -> ((i32, i32), (i32, i32)) {
    let distance = (right.0 - left.0, right.1 - left.1);

    (
        (left.0 - distance.0, left.1 - distance.1),
        (right.0 + distance.0, right.1 + distance.1),
    )
}

pub fn task1(antennas: &HashMap<char, Vec<(i32, i32)>>, bounds: (i32, i32)) -> usize {
    let mut antinodes = HashSet::<(i32, i32)>::new();

    for (_, locations) in antennas.iter() {
        for (left, right) in locations.iter().tuple_combinations() {
            let (back, forward) = reflect_antennas(left, right);
            if within_bounds(back, bounds) {
                antinodes.insert(back);
            }
            if within_bounds(forward, bounds) {
                antinodes.insert(forward);
            }
        }
    }

    antinodes.len()
}

struct Stepper {
    position: (i32, i32),
    bounds: (i32, i32),
    step: (i32, i32),
}

impl Iterator for Stepper {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if !within_bounds(self.position, self.bounds) {
            return None;
        }
        let prev = self.position;
        self.position = (self.position.0 + self.step.0, self.position.1 + self.step.1);
        Some(prev)
    }
}

fn step_along_direction(direction: (i32, i32)) -> (i32, i32) {
    let min_step = gcd::binary_u32(direction.0.abs() as u32, direction.1.abs() as u32);
    (direction.0 / min_step as i32, direction.1 / min_step as i32)
}

fn step_forward(from: (i32, i32), within: (i32, i32), direction: (i32, i32)) -> Stepper {
    Stepper {
        position: from,
        bounds: within,
        step: step_along_direction(direction),
    }
}

fn step_backward(from: (i32, i32), within: (i32, i32), direction: (i32, i32)) -> Stepper {
    Stepper {
        position: from,
        bounds: within,
        step: step_along_direction((-direction.0, -direction.1)),
    }
}

pub fn task2(antennas: &HashMap<char, Vec<(i32, i32)>>, bounds: (i32, i32)) -> usize {
    let mut antinodes = HashSet::<(i32, i32)>::new();

    for (_, locations) in antennas.iter() {
        for (left, right) in locations.iter().tuple_combinations() {
            let distance = (right.0 - left.0, right.1 - left.1);

            step_forward(*left, bounds, distance)
                .chain(step_backward(*left, bounds, distance))
                .for_each(|p| {
                    antinodes.insert(p);
                });
        }
    }

    antinodes.len()
}
