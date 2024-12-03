use regex::Regex;

pub fn task1(input: &str) -> i32 {
    Regex::new(r"mul\([0-9]{1,3}\,[0-9]{1,3}\)")
        .unwrap()
        .find_iter(input)
        .flat_map(|m| {
            match m
                .as_str()
                .strip_prefix("mul(")?
                .strip_suffix(")")?
                .split(",")
                .flat_map(|m| m.parse::<i32>())
                .collect::<Vec<i32>>()
                .as_slice()
            {
                [a, b] => Some(a * b),
                _ => None,
            }
        })
        .sum()
}

enum Operation {
    Do,
    Dont,
    Mul(i32),
}

pub fn task2(input: &str) -> i32 {
    return Regex::new(r"(mul\([0-9]{1,3}\,[0-9]{1,3}\))|(do\(\))|(don't\(\))")
        .unwrap()
        .find_iter(input)
        .flat_map(|m| match m.as_str() {
            "do()" => Some(Operation::Do),
            "don't()" => Some(Operation::Dont),
            value => match value
                .strip_prefix("mul(")?
                .strip_suffix(")")?
                .split(",")
                .flat_map(|m| m.parse::<i32>())
                .collect::<Vec<i32>>()
                .as_slice()
            {
                [a, b] => Some(Operation::Mul(a * b)),
                _ => None,
            },
        })
        .fold((0, true), |(sum, mul_enabled), op| {
            match (op, mul_enabled) {
                (Operation::Do, _) => (sum, true),
                (Operation::Dont, _) => (sum, false),
                (Operation::Mul(value), true) => (sum + value, true),
                _ => (sum, mul_enabled),
            }
        })
        .0;
}
