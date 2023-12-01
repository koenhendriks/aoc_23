use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Need an txt file to parse.");
        println!("Usage: aoc23_day_one input.txt");
        return Ok(())
    }

    // Open the file in read-only mode
    let file = File::open(args[1].to_string())?;
    let reader = BufReader::new(file);

    let mut sum:i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                sum = sum + filter_first_last_integer(&content);
            }
            Err(err) => {
                eprintln!("Error reading line {}: {}", index + 1, err);
            }
        }
    }

    println!("Answer: {}", sum);
    Ok(())
}

fn filter_first_last_integer(input: &String) -> i32 {
    let mut number: String = String::from("");

    for char in input.chars() {
        if char.is_numeric() {
            number.push(char);
            let _first_int = char.to_digit(10);
            break;
        }
    }

    for char in input.chars().rev(){
        if char.is_numeric() {
            number.push(char);
            let _last_int = char.to_digit(10);
            break;
        }
    }

    if number.is_empty(){
        return 0;
    }

    return number.parse::<i32>().unwrap();
}
