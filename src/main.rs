use std::fs;
use std::iter::FromIterator;

fn performActions(instructions: Vec<(char, i32)>) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut waypoint_x: i32 = 10;
    let mut waypoint_y: i32 = 1;
    //directions are 0 - east, 1 - north, 2 - west, 3 - south
    let mut current_heading = 0;

    for instruction in instructions {
        if instruction.0 == 'N' {
            y += instruction.1;
        } else if instruction.0 == 'S' {
            y -= instruction.1;
        } else if instruction.0 == 'E' {
            x += instruction.1;
        } else if instruction.0 == 'W' {
            x -= instruction.1;
        } else if instruction.0 == 'L' {
            current_heading = (current_heading + instruction.1 / 90) % 4;
        } else if instruction.0 == 'R' {
            current_heading = (current_heading + 4 - instruction.1 / 90) % 4;
        } else if instruction.0 == 'F' {
            match current_heading {
                0 => x += instruction.1,
                1 => y += instruction.1,
                2 => x -= instruction.1,
                3 => y -= instruction.1,
                _ => println!("something went wrong"),
            }
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
