use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    let mut items1 = Vec::<i32>::new();
    let mut items2 = Vec::<i32>::new();

    while io::stdin().read_line(&mut buffer)? > 0 {
        let tokens: Vec<&str> = buffer.split_whitespace().collect();

        items1.push(tokens[0].parse::<i32>().unwrap());
        items2.push(tokens[1].parse::<i32>().unwrap());

        buffer.clear();
    }

    println!("{}", day01::task2_fastest(&mut items1, &mut items2));
    Ok(())
}
