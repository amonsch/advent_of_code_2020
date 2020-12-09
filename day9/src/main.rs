use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn input(filename: &str) -> Result<Vec<i64>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut input: Vec<i64> = Vec::new();
    for num in reader.lines().map(|x| x.unwrap().parse::<i64>().unwrap()) {
        input.push(num);
    }
    Ok(input)
}

fn find_first_non_sum(preamble_len: usize, values: &[i64]) -> Option<i64> {
    let mut result = None;
    'outer: for i in preamble_len..values.len() {
        let mut is_combo = false;
        'middle: for j in i - preamble_len..i {
            for k in i - preamble_len..i {
                if j != k && values[i] == values[j] + values[k] {
                    is_combo = true;
                    break 'middle;
                }
            }
        }
        if !is_combo {
            result = Some(values[i]);
            break 'outer;
        }
    }
    result
}

fn find_consecutive_sum(number: i64, values: &[i64]) -> Option<i64> {
    let mut result = None;
    let mut i = 0;
    'outer: loop {
        // break if at end
        if i >= values.len() - 1 {
            break;
        }

        let mut sum: i64 = values[i];
        let mut offset = i + 1;

        // break if value bigger as number encountered while sum is still less than number
        if values[offset] > number {
            i = offset;
            continue;
        }

        loop {
            sum += values[offset];
            if sum == number {
                result = Some(
                    values[i..offset].iter().min().unwrap()
                        + values[i..offset].iter().max().unwrap(),
                );
                break 'outer;
            }

            // break if sum is bigger than the number we are looking for
            if sum > number {
                break;
            }
            offset += 1;
        }

        i += 1;
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("No input file or preamble length given!");
        return;
    }
    let preamble_len = args[2].parse::<usize>().unwrap();
    let input = input(&args[1]).unwrap();

    let result = find_first_non_sum(preamble_len, &input).unwrap();
    println!("Part I: {:?}", result);
    println!(
        "Part II: {:?}",
        find_consecutive_sum(result, &input).unwrap()
    );
}
