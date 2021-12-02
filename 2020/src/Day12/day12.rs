use std::fs;
use std::iter::FromIterator;

fn performActions(instructions: Vec<(char, i32)>) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut waypoint_x: i32 = 10;
    let mut waypoint_y: i32 = 1;
    //directions are 0 - east, 1 - north, 2 - west, 3 - south

    for instruction in instructions {
        if instruction.0 == 'N' {
            waypoint_y += instruction.1;
        } else if instruction.0 == 'S' {
            waypoint_y -= instruction.1;
        } else if instruction.0 == 'E' {
            waypoint_x += instruction.1;
        } else if instruction.0 == 'W' {
            waypoint_x -= instruction.1;
        } else if instruction.0 == 'R' {
            //Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
            let mut cos_value = 0;
            let mut sin_value = 0;
            match instruction.1 / 90 {
                0 => {
                    cos_value = 1;
                    sin_value = 0
                }
                1 => {
                    sin_value = 1;
                    cos_value = 0;
                }
                2 => {
                    sin_value = 0;
                    cos_value = -1;
                }
                3 => {
                    sin_value = -1;
                    cos_value = 0;
                }
                _ => println!("something went wrong"),
            }
            println!("waypoint was at {}, {}", waypoint_x, waypoint_y);
            let old_waypoint_x = waypoint_x;
            waypoint_x = cos_value * waypoint_x + sin_value * waypoint_y;
            waypoint_y = -sin_value * old_waypoint_x + cos_value * waypoint_y;

            println!("waypoint is now at {}, {}", waypoint_x, waypoint_y);
        } else if instruction.0 == 'L' {
            let mut cos_value = 0;
            let mut sin_value = 0;
            match instruction.1 / 90 {
                0 => {
                    cos_value = 1;
                    sin_value = 0
                }
                1 => {
                    sin_value = 1;
                    cos_value = 0;
                }
                2 => {
                    sin_value = 0;
                    cos_value = -1;
                }
                3 => {
                    sin_value = -1;
                    cos_value = 0;
                }
                _ => println!("something went wrong"),
            }
            println!("waypoint was at {}, {}", waypoint_x, waypoint_y);
            let old_waypoint_x = waypoint_x;
            waypoint_x = cos_value * waypoint_x - sin_value * waypoint_y;
            waypoint_y = sin_value * old_waypoint_x + cos_value * waypoint_y;

            println!(
                "waypoint is now at {}, {}, with cos {} and sin {}",
                waypoint_x, waypoint_y, cos_value, sin_value
            );
        } else if instruction.0 == 'F' {
            x += instruction.1 * waypoint_x;
            y += instruction.1 * waypoint_y;

            println!(
                "ship is now at {},{}, waypoint was at {},{}, with scalar {}",
                x, y, waypoint_x, waypoint_y, instruction.1
            );
        }
    }
    return (x, y);
}

fn main() {
    // --snip
    let filename = "src/Day12/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut instructions: Vec<(char, i32)> = Vec::new();

    for line in lines {
        let char_vec: Vec<char> = line.chars().collect();
        let value = String::from_iter(&char_vec[1..]).parse().unwrap();
        instructions.push((char_vec[0], value));
    }
    let result = performActions(instructions.clone());
    println!("manhatten distance is {}", result.0.abs() + result.1.abs());
}
