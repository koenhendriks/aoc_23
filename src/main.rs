use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file in read-only mode
    let file = File::open("one/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum:i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                println!("Line {}: {} -> {:?}", index + 1, content, filter_first_last_integer(&content));
                sum = sum + filter_first_last_integer(&content);
            }
            Err(err) => {
                eprintln!("Error reading line {}: {}", index + 1, err);
            }
        }
    }

    println!("Total sum: {}", sum);
    Ok(())
}

fn filter_first_last_integer(input: &String) -> i32 {
    let mut first_int: Option<u32> = None;
    let mut last_int: Option<u32> = None;
    let mut number: String = String::from("");

    for char in input.chars() {
        if char.is_numeric() {
            number.push(char);
            first_int = char.to_digit(10);
            break;
        }
    }

    for char in input.chars().rev(){
        if char.is_numeric() {
            number.push(char);
            last_int = char.to_digit(10);
            break;
        }
    }

    if number.is_empty(){
        return 0;
    }

    return number.parse::<i32>().unwrap();
}
