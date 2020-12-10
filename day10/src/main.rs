use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use std::collections::HashMap;

fn input(filename: &str) -> Result<Vec<i64>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut input: Vec<i64> = Vec::new();
    for num in reader.lines().map(|x| x.unwrap().parse::<i64>().unwrap()) {
        input.push(num);
    }
    Ok(input)
}

fn calc_diff_prod(values: &[i64]) -> i64 {
    let mut one_count = 1;
    let mut three_count = 1;
    for (i, value) in values.iter().enumerate().skip(1) {
        match value - values[i - 1] {
            1 => one_count += 1,
            3 => three_count += 1,
            _ => {}
        }
    }
    one_count * three_count
}

fn calc_combinations(current: i64, max: i64, values: &[i64], cache: &mut HashMap<i64, i64>) -> i64 {
    let mut total = 0;
    for i in 1..=3 {
        let diff = current + i;
        if diff == max {
            total += 1;
        } else if diff < max && values.contains(&diff) {
            if let Some(count) = cache.get(&diff) {
                total += count
            } else {
                let count = calc_combinations(diff, max, values, cache);
                total += count;
                cache.insert(diff, count);
            }
        }
    }
    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file given!");
        return;
    }
    let mut values = input(&args[1]).unwrap();
    values.sort_unstable();
    println!("Part I: {:?}", calc_diff_prod(&values));
    let mut cache: HashMap<i64, i64> = HashMap::new();
    println!(
        "Part II: {:?}",
        calc_combinations(0, *values.iter().max().unwrap() + 3, &values, &mut cache)
    );
}
