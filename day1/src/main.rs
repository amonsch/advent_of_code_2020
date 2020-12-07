use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn input(filename: &str) -> Result<Vec<i64>> {
    let file = File::open(filename)?;
    let result: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect();
    Ok(result)
}

fn find_two(input_filename: &str) {
    if let Ok(nums) = input(input_filename) {
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == 2020 {
                    println!("idxs: {} {}", i, j);
                    println!("vals: {} {}", nums[i], nums[j]);
                    println!("{}", nums[i] * nums[j]);
                    break;
                }
            }
        }
    }
}

fn find_three(input_filename: &str) {
    if let Ok(nums) = input(input_filename) {
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 2020 {
                        println!("idxs: {} {} {}", i, j, k);
                        println!("vals: {} {} {}", nums[i], nums[j], nums[k]);
                        println!("result: {}", nums[i] * nums[j] * nums[k]);
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No input file given!");
        return;
    }

    println!("Looking for 2");
    find_two(&args[1]);
    println!();

    println!("Looking for 3");
    find_three(&args[1]);
    println!();
}
