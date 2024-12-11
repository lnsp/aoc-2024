use std::collections::HashMap;

fn count_stones(stones: &Vec<usize>, iterations: usize) -> usize {
    let mut snapshot: HashMap<usize, usize> = HashMap::new();
    for &stone in stones {
        *snapshot.entry(stone).or_insert(0) += 1;
    }

    let mut next_inserts: Vec<(usize, usize)> = vec![];
    for _ in 0..iterations {
        next_inserts.clear();
        for (&value, &count) in &snapshot {
            let value_str = value.to_string();
            if value == 0 {
                next_inserts.push((1, count));
            } else if value_str.len() % 2 == 0 {
                let (left, right) = (
                    &value_str[..(value_str.len() / 2)].parse::<usize>().unwrap(),
                    &value_str[(value_str.len() / 2)..].parse::<usize>().unwrap(),
                );
                next_inserts.push((*left, count));
                next_inserts.push((*right, count));
            } else {
                next_inserts.push((value * 2024, count));
            }
        }
        snapshot.clear();
        for (value, count) in &next_inserts {
            *snapshot.entry(*value).or_insert(0) += count;
        }
    }

    snapshot.iter().map(|(_, count)| count).sum()
}


pub fn task1(stones: &Vec<usize>) -> usize {
    count_stones(stones, 25)
}

pub fn task2(stones: &Vec<usize>) -> usize {
    count_stones(stones, 75)
}