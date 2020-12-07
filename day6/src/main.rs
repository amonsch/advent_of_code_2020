use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn input(filename: &str) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut tmp: String = String::from("");
    for line in reader.lines() {
        let value = line.unwrap();
        if value != "" {
            tmp = format!("{} {}", tmp, value);
        } else {
            result.push(tmp.trim().to_string());
            tmp = "".to_string();
        }
    }

    if tmp != "" {
        result.push(tmp.trim().to_string());
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file given!");
        return;
    }

    let lines: Vec<String> = input(&args[1]).unwrap();
    let mut sum = 0;
    for line in &lines {
        let mut result: Vec<char> = line.chars().collect();
        result.retain(|c| !c.is_whitespace());
        result.sort_unstable();
        result.dedup();
        sum += result.len();
    }
    println!("Part I {}", sum);

    let mut all = 0;
    for line in &lines {
        let mut state: [usize; 26] = [0; 26];
        let strings: Vec<&str> = line.split(' ').collect();
        for string in &strings {
            for ch in string.chars() {
                let idx = ch as usize - 97;
                state[idx] += 1;
                if state[idx] == strings.len() {
                    all += 1;
                }
            }
        }
    }
    println!("Part II {}", all);
}
