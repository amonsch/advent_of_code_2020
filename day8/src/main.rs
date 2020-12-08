use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug)]
enum OpCode {
    NOP,
    ACC,
    JMP,
}

fn input(filename: &str) -> Result<Vec<(OpCode, i32)>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut code: Vec<(OpCode, i32)> = Vec::new();
    for line in reader.lines().map(|x| x.unwrap()) {
        let mut iter = line.split(' ');
        let op = iter.next().unwrap();
        let opcode = match op {
            "nop" => OpCode::NOP,
            "acc" => OpCode::ACC,
            "jmp" => OpCode::JMP,
            _ => panic!("Unknown opcode encountered"),
        };
        let arg = iter.next().unwrap();
        code.push((opcode, arg.to_string().parse::<i32>().unwrap()));
    }
    Ok(code)
}

fn acc_value_before_loop(code: &[(OpCode, i32)]) -> i32 {
    let mut acc = 0;
    let mut offset = 0;
    let mut opcode = &code[offset].0;
    let mut arg = code[offset].1;
    let mut visited = vec![false; code.len()];
    loop {
        visited[offset] = true;
        match opcode {
            OpCode::NOP => {
                offset += 1;
            }
            OpCode::ACC => {
                offset += 1;
                acc += arg;
            }
            OpCode::JMP => {
                offset = (offset as i32 + arg) as usize;
            }
        }

        if visited[offset] {
            break;
        }

        opcode = &code[offset].0;
        arg = code[offset].1;
    }
    acc
}

fn acc_value_before_fix(
    flip_offset: i32,
    flip_opcode: &OpCode,
    code: &[(OpCode, i32)],
) -> Option<i32> {
    let mut acc = 0;
    let mut offset = 0;
    let mut opcode = &code[offset].0;
    let mut arg = code[offset].1;
    let mut visited = vec![false; code.len()];
    loop {
        visited[offset] = true;

        if offset == flip_offset as usize {
            opcode = match flip_opcode {
                OpCode::JMP => &OpCode::NOP,
                OpCode::NOP => &OpCode::JMP,
                _ => panic!("Oh noe"),
            }
        }

        match opcode {
            OpCode::NOP => {
                offset += 1;
            }
            OpCode::ACC => {
                offset += 1;
                acc += arg;
            }
            OpCode::JMP => {
                offset = (offset as i32 + arg) as usize;
            }
        }

        if offset == code.len() {
            return Some(acc);
        }

        if visited[offset] {
            break;
        }

        opcode = &code[offset].0;
        arg = code[offset].1;
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file given!");
        return;
    }

    let code = input(&args[1]).unwrap();
    println!("ACC before loop: {:?}", acc_value_before_loop(&code));

    for (offset, (opcode, _)) in code.iter().enumerate() {
        match opcode {
            OpCode::NOP | OpCode::JMP => {
                if let Some(result) = acc_value_before_fix(offset as i32, opcode, &code) {
                    println!("ACC before fix: {:?}", result);
                }
            }
            _ => continue,
        }
    }
}
