use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::RangeInclusive;

use regex::Regex;

const BYR_RANGE: RangeInclusive<u32> = 1920..=2002;
const IYR_RANGE: RangeInclusive<u32> = 2010..=2020;
const EYR_RANGE: RangeInclusive<u32> = 2020..=2030;
const HGT_CM_RANGE: RangeInclusive<u32> = 150..=193;
const HGT_IN_RANGE: RangeInclusive<u32> = 59..=76;

const ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

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
            let passport: &str = tmp.trim();
            result.push(passport.to_string());
            tmp = "".to_string();
        }
    }

    Ok(result)
}

fn basic_validation(passports: &[String]) -> u32 {
    let mut valid = 0;
    for pass in passports {
        let mut fields: Vec<&str> = pass
            .split(' ')
            .map(|x| x.split(':').next().unwrap())
            .collect();
        fields.sort_unstable();
        let fields_str = fields.join("");
        if fields_str == "byrcidecleyrhclhgtiyrpid" || fields_str == "byrecleyrhclhgtiyrpid" {
            valid += 1;
        }
    }

    valid
}

fn advanced_validation(passports: &[String]) -> u32 {
    let mut valid = 0;
    let hcl: Regex = Regex::new(r"^#[0-9a-f]").unwrap();
    let pid: Regex = Regex::new(r"^[\d]{9}$").unwrap();
    for pass in passports {
        let mut validation = 0b0000000;
        for mut field in pass.split(' ').map(|x| x.split(':')) {
            let name = field.next().unwrap();
            let value = field.next().unwrap();
            match name {
                "byr" => {
                    if BYR_RANGE.contains(&value.parse::<u32>().unwrap()) {
                        validation |= 0b1000000;
                    } else {
                        break;
                    }
                }
                "iyr" => {
                    if IYR_RANGE.contains(&value.parse::<u32>().unwrap()) {
                        validation |= 0b0100000;
                    } else {
                        break;
                    }
                }
                "eyr" => {
                    if EYR_RANGE.contains(&value.parse::<u32>().unwrap()) {
                        validation |= 0b0010000;
                    } else {
                        break;
                    }
                }
                "hgt" => {
                    if !(value.ends_with("cm") || value.ends_with("in")) {
                        break;
                    }
                    let (height, unit) = value.split_at(value.len() - 2);
                    match unit {
                        "cm" => {
                            if HGT_CM_RANGE.contains(&height.parse::<u32>().unwrap()) {
                                validation |= 0b0001000;
                            } else {
                                break;
                            }
                        }
                        "in" => {
                            if HGT_IN_RANGE.contains(&height.parse::<u32>().unwrap()) {
                                validation |= 0b0001000;
                            } else {
                                break;
                            }
                        }
                        _ => {
                            println!("height {:?} value {} - unit {}", height, value, unit);
                            panic!("Unknown height unit")
                        }
                    }
                }
                "hcl" => {
                    if hcl.is_match(value) {
                        validation |= 0b0000100;
                    } else {
                        break;
                    }
                }
                "ecl" => {
                    if ECL.contains(&value) {
                        validation |= 0b0000010;
                    } else {
                        break;
                    }
                }
                "pid" => {
                    if pid.is_match(value) {
                        validation |= 0b0000001;
                    } else {
                        break;
                    }
                }
                "cid" => {
                    // ignore
                }
                _ => {
                    panic!("Unknown field")
                }
            }
        }
        if validation == 0b1111111 {
            valid += 1;
        }
    }
    valid
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file given!");
        return;
    }

    let valid = basic_validation(&input(&args[1]).unwrap());
    println!("Valid passports (basic check): {}", valid);

    println!(
        "Valid passports (advanced check): {}",
        advanced_validation(&input(&args[1]).unwrap())
    );
}
