use std::fs;

fn main() {
    // --snip
    let filename = "src/Day8/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let instructions: Vec<&str> = contents.lines().collect();

    let mut parsed_instructions: Vec<(&str, i32)> = Vec::new();

    for instruction in instructions {
        let values: Vec<&str> = instruction.split(" ").collect();
        let offset: i32 = values[1].parse().unwrap();
        parsed_instructions.push((values[0], offset));
        //println!("got instruction {}, with offset {}", values[0], offset);
    }

    let mut tested_mutated_instructions: Vec<i32> = Vec::new();
    let mut solution_found = false;

    loop {
        let mut total_accumulator = 0;
        let mut current_program_counter: isize = 0;
        let mut visited_indices: Vec<i32> = Vec::new();
        let mut mutated_already = false;
        if solution_found {
            break;
        }
        println!("Search for {}", parsed_instructions.len() + 1);
        loop {
            println!("tested {} mutations", tested_mutated_instructions.len());
            if visited_indices
                .iter()
                .any(|&i| i as usize == current_program_counter as usize)
            {
                println!("Stop at line {}", current_program_counter);
                break;
            }
            println!("Current program counter is {}", current_program_counter);
            if current_program_counter as usize == parsed_instructions.len() {
                println!("FOUND SOLUTION, break here with acc {}", total_accumulator);
                solution_found = true;
                break;
            }
            visited_indices.push(current_program_counter as i32);
            let mut instruction = parsed_instructions[current_program_counter as usize];
            if !tested_mutated_instructions
                .iter()
                .any(|&i| i as usize == current_program_counter as usize)
                && !mutated_already
            {
                if instruction.0 == "nop" {
                    instruction.0 = "jmp";
                    mutated_already = true;
                    println!(
                        "Changed instruction {} with offset {}",
                        instruction.0, instruction.1
                    );
                    tested_mutated_instructions.push(current_program_counter as i32);
                } else if instruction.0 == "jmp" {
                    instruction.0 = "nop";
                    mutated_already = true;
                    println!(
                        "Changed instruction {} with offset {}",
                        instruction.0, instruction.1
                    );
                    tested_mutated_instructions.push(current_program_counter as i32);
                }
            }
            if instruction.0 == "nop" {
                current_program_counter += 1;
                continue;
            } else if instruction.0 == "acc" {
                total_accumulator += instruction.1;
                current_program_counter += 1;
                continue;
            } else if instruction.0 == "jmp" {
                current_program_counter += instruction.1 as isize;
                continue;
            }
        }
    }
}
