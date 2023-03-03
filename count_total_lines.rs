// This program reads a file and counts the number of lines of code in it.

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: line_counter <file>");
        return;
    }

    let contents = match fs::read_to_string(&args[1]) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Error reading file: {}", error);
            return;
        }
    };

    let line_count = contents.lines().count();
    println!("Number of lines: {}", line_count);
}


