use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug)]
struct Check {
    pub a: u32,
    pub b: u32,
    pub c: char,
    pub password: String,
}

fn input(filename: &str) -> Result<Vec<Check>> {
    let file = File::open(filename)?;
    let mut result: Vec<Check> = Vec::new();
    for line in BufReader::new(file).lines() {
        let value = line.unwrap();
        let check: Vec<&str> = value.split(": ").collect();
        let policy: Vec<&str> = check[0].split(' ').collect();
        let c = policy[1].chars().next().unwrap();
        let min_max: Vec<u32> = policy[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        result.push(Check {
            c,
            a: min_max[0],
            b: min_max[1],
            password: check[1].to_string(),
        });
    }
    Ok(result)
}

fn check_occurance(input_filename: &str) {
    let mut valid = 0;
    for check in input(input_filename).unwrap() {
        let mut count = 0;
        for ch in check.password.chars() {
            if ch == check.c {
                count += 1;
                if count > check.b {
                    break;
                }
            }
        }
        if count >= check.a && count <= check.b {
            valid += 1;
            // println!("Valid {:?}", check)
        }
    }
    println!("Valid count {}", valid);
}

fn check_position(input_filename: &str) {
    let mut valid = 0;
    for check in input(input_filename).unwrap() {
        let a = check.password.chars().nth((check.a - 1) as usize).unwrap();
        let b = check.password.chars().nth((check.b - 1) as usize).unwrap();
        if (a == check.c && b != check.c) || (b == check.c && a != check.c)
        {
            // println!("Valid {:?}", check);
            valid += 1;
        }
    }
    println!("Valid count {}", valid);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No input file given");
        return;
    }

    // check_occurance(&args[1]);
    check_position(&args[1]);
}
