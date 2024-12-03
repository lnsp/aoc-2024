use std::cmp::{max, min};
use std::collections::HashMap;

pub fn task2(items1: &mut Vec<i32>, items2: &mut Vec<i32>) -> i32 {
    let items2freq = items2.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let mut sum: i32 = 0;
    for a in items1.iter() {
        let f: i32 = match items2freq.get(a) {
            Some(v) => *v,
            None => 0,
        };
        sum += a * f;
    }

    sum
}

pub fn task2_faster(items1: &mut Vec<i32>, items2: &mut Vec<i32>) -> i32 {
    for value in items1.iter_mut() {
        *value = *value * (items2.iter().filter(|v| *v == value).count() as i32)
    }
    items1.iter().sum()
}

fn reduce_to_count(items: &mut Vec<i32>) -> Vec<(i32, i32)> {
    items.sort();
    items.iter().fold(Vec::new(), |mut v, i| {
        match v.last_mut() {
            Some((last_value, last_freq)) => {
                if last_value == i {
                    *last_freq += 1;
                } else {
                    v.push((*i, 1));
                }
            }
            None => {
                v.push((*i, 1));
            }
        };
        v
    })
}

pub fn task2_fastest(items1: &mut Vec<i32>, items2: &mut Vec<i32>) -> i32 {
    items1.sort();
    items2.sort();
    let items1cp = reduce_to_count(items1);
    let items2cp = reduce_to_count(items2);
    let (mut sum, mut i, mut j): (i32, usize, usize) = (0, 0, 0);
    while i < items1cp.len() && j < items2cp.len() {
        if items1cp[i].0 < items2cp[j].0 {
            i += 1;
        } else if items1cp[i].0 > items2cp[j].0 {
            j += 1;
        } else {
            sum += items1cp[i].0 * items1cp[i].1 * items2cp[j].1;
            i += 1;
            j += 1;
        }
    }
    sum
}

pub fn task1(items1: &mut Vec<i32>, items2: &mut Vec<i32>) -> i32 {
    items1.sort();
    items2.sort();

    let mut sum: i32 = 0;
    for (a, b) in items1.iter().zip(items2.iter()) {
        sum += max(a, b) - min(a, b);
    }
    sum
}
