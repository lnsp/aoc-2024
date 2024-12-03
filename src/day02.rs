pub fn task1(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .filter(|report| task1_valid_report(report))
        .count() as i32
}

fn task1_valid_report(report: &Vec<i32>) -> bool {
    report[1..]
        .iter()
        .fold(
            (report[0], None, true),
            |(last_value, last_step, valid), value| {
                let step = value - last_value;
                if !valid
                    || abs(step) < 1
                    || abs(step) > 3
                    || match last_step {
                        None => false,
                        Some(last_step) => sgn(step) != sgn(last_step),
                    }
                {
                    (*value, None, false)
                } else {
                    (*value, Some(step), true)
                }
            },
        )
        .2
}

pub fn task2(reports: &Vec<Vec<i32>>) -> i32 {
    reports
        .iter()
        .filter(|report| {
            if task1_valid_report(report) {
                return true;
            }
            for i in 0..report.len() {
                let spliced_report: Vec<i32> = report[..i]
                    .iter()
                    .chain(report[i + 1..].iter())
                    .map(|v| *v)
                    .collect();
                if task1_valid_report(&spliced_report) {
                    return true;
                }
            }
            false
        })
        .count() as i32
}

pub fn abs(v: i32) -> i32 {
    if v < 0 { -v } else { v }
}

pub fn sgn(v: i32) -> i32 {
    if v < 0 {
        -1
    } else if v > 0 {
        1
    } else {
        0
    }
}
