use std::fs::File;
use std::io::{BufRead, BufReader}; // 0.9.0

const WORLD_SIZE: usize = 1002;

fn read_file() -> Vec<String> {
    let f = BufReader::new(File::open("src/input.txt").unwrap());
    //862,121 -> 14,969
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    return lines;
}

fn create_world(lines: Vec<String>) -> Vec<u8> {
    let len = WORLD_SIZE * WORLD_SIZE;
    let mut world: Vec<u8> = vec![0; len];
    for l in lines {
        //[862,121;14,969]
        let left_right: Vec<&str> = l.split(" -> ").collect::<Vec<&str>>();
        let mut line: Vec<u32> = vec![];
        let mut x: u32;
        let mut y: u32;
        for string in left_right {
            let xy: Vec<&str> = string.split(",").collect();
            x = xy[0].to_string().parse::<u32>().expect("invalid input");
            line.push(x);
            y = xy[1].to_string().parse::<u32>().expect("invalid input");
            line.push(y);
        }
        print!(
            "x1: {}, y1: {}, x2: {}, y2: {}",
            line[0], line[1], line[2], line[3]
        );
        let x_equals = line[0] == line[2];
        let y_equals = line[1] == line[3];
        let dir_x_positive = line[0] < line[2];
        let dir_y_positive = line[1] < line[3];
        if x_equals && !y_equals {
            let x = line[0];
            if dir_y_positive {
                for y in line[1]..line[3] + 1 {
                    world[(x * WORLD_SIZE as u32 + y) as usize] += 1;
                }
            } else {
                for y in line[3]..line[1] + 1 {
                    world[(x * WORLD_SIZE as u32 + y) as usize] += 1;
                }
            }
            println!();
            print_world(world.clone());
        } else if !x_equals && y_equals {
            let y = line[1];
            if dir_x_positive {
                for x in line[0]..line[2] + 1 {
                    world[(x * WORLD_SIZE as u32 + y) as usize] += 1;
                }
            } else {
                for x in line[2]..line[0] + 1 {
                    world[(x * WORLD_SIZE as u32 + y) as usize] += 1;
                }
            }
            println!();
            print_world(world.clone());
        } else {
            println!(" -> diagonal line");
            if dir_x_positive {
                let mut y: u32 = line[1];
                let mut dir_y: i32 = 0;
                if dir_y_positive {
                    dir_y = 1;
                } else {
                    dir_y = -1;
                }
                for x in line[0]..line[2] + 1 {
                    world[(x * WORLD_SIZE as u32 + y) as usize] += 1;
                    y = (y as i32 + dir_y) as u32;
                }
            } else {
                let mut y: u32 = line[3];
                let mut dir_y: i32 = 0;
                if !dir_y_positive {
                    dir_y = 1;
                } else {
                    dir_y = -1;
                }
                for x in line[2]..line[0] + 1 {
                    world[(x * WORLD_SIZE as u32 + y) as usize] += 1;
                    y = (y as i32 + dir_y) as u32;
                }
            }
            println!();
            print_world(world.clone());
        }
    }
    return world;
}

fn print_world(world: Vec<u8>) {
    // println!("Current world state!");
    // println!("\t 0  1  2  3  4  5  6  7  8  9  10");
    // println!();
    // for x in 0..WORLD_SIZE - 1 {
    //     print!("{}\t", x);
    //     for y in 0..WORLD_SIZE - 1 {
    //         print!(" {} ", world[x + WORLD_SIZE * y]);
    //     }
    //     println!();
    // }
}

fn count_world(world: Vec<u8>) {
    let mut total = 0;
    for x in 0..WORLD_SIZE - 1 {
        for y in 0..WORLD_SIZE - 1 {
            if world[x + WORLD_SIZE * y] > 1 {
                total += 1;
            }
        }
    }
    println!("total was {}", total);
}

fn main() {
    let lines: Vec<String> = read_file();
    let world: Vec<u8> = create_world(lines.clone());
    print_world(world.clone());
    count_world(world.clone());
}
