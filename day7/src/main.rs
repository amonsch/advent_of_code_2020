use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn input(filename: &str) -> Result<HashMap<String, Vec<(usize, String)>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut bags: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for line in reader.lines() {
        let original = line.unwrap();
        let strings: Vec<&str> = original.split(" contain ").collect();
        let bag: String = strings[0].rsplitn(2, ' ').skip(1).collect();
        let mut subbags: Vec<(usize, String)> = Vec::new();
        for subbag_string in strings[1].split(", ") {
            let mut iter = subbag_string.splitn(2, ' ');
            let mut count = 0;
            let mut subbag: String = String::from("None");
            if let Ok(factor) = iter.next().unwrap().parse::<usize>() {
                count = factor;
                subbag = iter.next().unwrap().rsplitn(2, ' ').skip(1).collect();
            }
            subbags.push((count, subbag.to_string()));
        }
        bags.insert(bag, subbags);
    }

    Ok(bags)
}

fn contains_bag(
    color: &str,
    bags: &[(usize, String)],
    map: &HashMap<String, Vec<(usize, String)>>,
) -> bool {
    for (_, bag) in bags {
        // empty
        if bag == "None" {
            return false;
        }

        // hit
        if bag == color {
            return true;
        }

        // subhit
        if contains_bag(color, map.get(bag).unwrap(), &map) {
            return true;
        }
    }
    false
}

fn contains_how_many(color: &str, map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut sum: usize = 0;
    if let Some(subbags) = map.get(color) {
        for (count, subbag) in subbags {
            sum += count + count * contains_how_many(subbag, map);
        }
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file given!");
        return;
    }

    let mut count = 0;
    let bags = input(&args[1]).unwrap();
    for subbags in bags.values() {
        if contains_bag("shiny gold", &subbags, &bags) {
            count += 1;
        }
    }
    println!("Count {}", count);

    println!(
        "shiny gold bag contians {:?}",
        contains_how_many("shiny gold", &bags)
    );
}
