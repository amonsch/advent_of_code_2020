use std::cmp;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn input(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let result: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file given!");
        return;
    }

    let mut values: Vec<usize> = Vec::new();
    let mut max: usize = 0;
    for line in input(&args[1]).unwrap() {
        let mut row_left = 0;
        let mut row_right = 127;

        let mut seat_left = 0;
        let mut seat_right = 7;
        for ch in line.chars() {
            match ch {
                'F' => row_right = (row_left + row_right) / 2,
                'B' => row_left = (row_left + row_right) / 2 + 1,
                'L' => seat_right = (seat_left + seat_right) / 2,
                'R' => seat_left = (seat_left + seat_right) / 2 + 1,
                _ => {}
            }
        }
        let value = row_left * 8 + seat_left;
        values.push(value);
        max = cmp::max(max, value);
    }
    println!("Max is {}", max);
    values.sort_unstable();

    let mut prev = 0;
    for value in values {
        if prev == 0 {
            prev = value;
            continue;
        }

        if value - prev > 1 {
            println!("Missing value: {}", value);
            break;
        }
        prev = value;
    }
}
