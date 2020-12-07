use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn input(filename: &str) -> Result<Vec<Vec<char>>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        result.push(Vec::new());
        for ch in line.unwrap().chars() {
            result[i].push(ch);
        }
    }

    Ok(result)
}

fn tree_count(grid: &[Vec<char>], x_slope: usize, y_slope: usize) -> u64 {
    let width = grid[0].len();
    let mut x = 0;
    let mut count = 0;
    for i in (y_slope..grid.len()).step_by(y_slope) {
        x = (x + x_slope) % width;
        if grid[i][x] == '#' {
            count += 1;
        }
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file given!");
        return;
    }

    let grid = input(&args[1]).unwrap();

    let total_tree_count = tree_count(&grid, 1, 1)
        * tree_count(&grid, 3, 1)
        * tree_count(&grid, 5, 1)
        * tree_count(&grid, 7, 1)
        * tree_count(&grid, 1, 2);

    println!("Total trees encountered: {}", total_tree_count);
}
