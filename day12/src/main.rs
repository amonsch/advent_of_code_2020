use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn input(filename: &str) -> Result<Vec<(char, i32)>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut input: Vec<(char, i32)> = Vec::new();
    for action in reader.lines().map(|x| x.unwrap()) {
        let (dir_str, value_str) = action.split_at(1);
        let dir = dir_str.chars().next().unwrap();
        let value = value_str.parse::<i32>().unwrap();
        input.push((dir, value));
    }
    Ok(input)
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_rotate() {
        let mut res = rotate(1.0, 0.0, (90.0 as f32).to_radians());
        assert_eq!((res.0 as i32, res.1 as i32), (0, -1));

        res = rotate(1.0, 0.0, (-90.0 as f32).to_radians());
        assert_eq!((res.0 as i32, res.1 as i32), (0, 1));
    }
}

fn rotate(x: f32, y: f32, rad: f32) -> (f32, f32) {
    let new_x: f32 = x * f32::cos(rad) + y * f32::sin(rad);
    let new_y: f32 = x * -f32::sin(rad) + y * f32::cos(rad);
    (new_x.round(), new_y.round())
}

fn calc_distance(move_waypoint: bool, mut dir_x: i32, mut dir_y: i32, actions: &[(char, i32)]) {
    let mut x = 0;
    let mut y = 0;

    for action in actions {
        match action.0 {
            'L' | 'R' => match action.1 {
                90 | 180 | 270 => {
                    let degree: f32;
                    if action.0 == 'R' {
                        degree = action.1 as f32;
                    } else {
                        degree = -action.1 as f32;
                    }
                    let new = rotate(dir_x as f32, dir_y as f32, degree.to_radians());
                    dir_x = new.0 as i32;
                    dir_y = new.1 as i32;
                }
                _ => {
                    panic!("Invalid orientation")
                }
            },
            'F' => {
                x += dir_x * action.1;
                y += dir_y * action.1;
            }
            'E' => {
                if !move_waypoint {
                    x += action.1;
                } else {
                    dir_x += action.1;
                }
            }
            'W' => {
                if !move_waypoint {
                    x -= action.1;
                } else {
                    dir_x -= action.1;
                }
            }
            'N' => {
                if !move_waypoint {
                    y += action.1;
                } else {
                    dir_y += action.1;
                }
            }
            'S' => {
                if !move_waypoint {
                    y -= action.1;
                } else {
                    dir_y -= action.1;
                }
            }
            _ => {
                panic!("Invalid action")
            }
        }
    }
    println!("Result: {}", i32::abs(x) + i32::abs(y));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file given!");
        return;
    }
    let actions = input(&args[1]).unwrap();
    calc_distance(false, 1, 0, &actions);
    calc_distance(true, 10, 1, &actions);
}
