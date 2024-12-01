use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use std::{
    io::Error,
    io::{self},
};

fn main() -> Result<(), Error> {
    let mut buffer = String::new();

    let mut items1 = Vec::<i32>::new();
    let mut items2 = Vec::<i32>::new();

    while io::stdin().read_line(&mut buffer)? > 0 {
        let tokens: Vec<&str> = buffer.split_whitespace().collect();

        items1.push(tokens[0].parse::<i32>().unwrap());
        items2.push(tokens[1].parse::<i32>().unwrap());

        buffer.clear();
    }

    /*
    items1.sort();
    items2.sort();
    */

    let items2freq = items2.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });

    let mut sum: i32 = 0;
    /* Task 1
    for (a, b) in items1.iter().zip(items2.iter()) {
        sum += max(a, b) - min(a, b);
    }
    */
    /* Task 2 */
    for a in items1.iter() {
        let f: i32 = match items2freq.get(a) {
            Some(v) => *v,
            None => 0,
        };
        sum += a * f;
    }

    println!("{}", sum);
    Ok(())
}
