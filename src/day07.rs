use std::cmp::min;

pub struct Equation {
    pub sum: i64,
    pub values: Vec<i64>,
}

impl Equation {
    fn evaluate(&self, operators: &Vec<Operator>) -> Option<i64> {
        let mut partial_sum = self.values[0];

        for index in 0..min(self.values.len() - 1, operators.len()) {
            if partial_sum > self.sum {
                return None;
            }
            partial_sum = operators[index].apply(partial_sum, self.values[index + 1])
        }
        Some(partial_sum)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            Operator::Concat => (left.to_string() + &right.to_string()).parse().unwrap(),
        }
    }
}

pub fn task1(equations: &Vec<Equation>) -> i64 {
    solve(equations, &[Operator::Add, Operator::Multiply])
}

pub fn task2(equations: &Vec<Equation>) -> i64 {
    solve(equations, &[Operator::Add, Operator::Multiply, Operator::Concat])
}

fn solve(equations: &Vec<Equation>, allowed_operators: &[Operator]) -> i64 {
    equations
        .iter()
        .filter(|eq| {
            let mut stack = Vec::<Vec<Operator>>::new();
            stack.push(vec![]);

            while !stack.is_empty() {
                let operators = stack.pop().unwrap();
                match eq.evaluate(&operators) {
                    Some(partial_sum) => {
                        if partial_sum == eq.sum && operators.len() == eq.values.len() - 1 {
                            return true;
                        } else if operators.len() == eq.values.len() - 1 {
                            continue;
                        }
                        for next_operator in allowed_operators {
                            let mut operators_clone = operators.clone();
                            operators_clone.push(*next_operator);
                            stack.push(operators_clone);
                        }
                    }
                    None => (),
                }
            }

            false
        })
        .map(|eq| eq.sum)
        .sum()
}
