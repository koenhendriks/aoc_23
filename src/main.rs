use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Get the current working directory
    let current_dir = env::current_dir()?;

    // Specify the relative path to the file
    let file_path = current_dir.join("one").join("input.txt");

    // Open the file in read-only mode
    let file = File::open(&file_path)?;

    // Create a buffered reader to efficiently read lines
    let reader = BufReader::new(file);

    // Iterate over the lines in the file
    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                println!("Line {}: {} -> {:?}", index + 1, content, filter_first_last_integer(&content));
            }
            Err(err) => {
                eprintln!("Error reading line {}: {}", index + 1, err);
            }
        }
    }

    Ok(())
}

fn filter_first_last_integer(input: &String) -> i32 {
    for char in input.chars() {
        if char.is_numeric() {
            // found first number.
        }
    }

    return 1;
}
