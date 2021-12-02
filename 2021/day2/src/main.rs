use std::fs::File;
use std::io::{BufRead, BufReader};

enum Command {
    Up,
    Down,
    Forward,
}

fn read_file() -> Vec<(Command, i32)> {
    let f = BufReader::new(File::open("src/input.txt").unwrap());
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    let commands: Vec<(Command, i32)> = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
            let val: i32 = parts[1].parse().unwrap();
            let instr = match parts[0] {
                "forward" => Command::Forward,
                "up" => Command::Up,
                "down" => Command::Down,
                _ => panic!("bruh"),
            };
            (instr, val)
        })
        .collect();
    return commands;
}
fn task1() {
    let commands: Vec<(Command, i32)> = read_file();
    let mut x_val = 0;
    let mut y_val = 0;
    for (com, val) in commands {
        match com {
            Command::Forward => x_val += val,
            Command::Up => y_val -= val,
            Command::Down => y_val += val,
        }
    }
    println!(
        "result was {} times {} equals {}",
        x_val,
        y_val,
        x_val * y_val
    );
}

fn task2() {
    let commands: Vec<(Command, i32)> = read_file();
    let mut x_val = 0;
    let mut y_val = 0;
    let mut aim = 0;
    for (com, val) in commands {
        match com {
            Command::Forward => {
                x_val += val;
                y_val += aim * val;
            }
            Command::Up => aim -= val,
            Command::Down => aim += val,
        }
    }
    println!(
        "result was {} times {} equals {}",
        x_val,
        y_val,
        x_val * y_val
    );
}
fn main() {
    // --snip--
    task1();
    task2();
    //task2(&split);
}
