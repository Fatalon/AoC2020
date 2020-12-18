use std::fs;

fn calculateRecursivly(mut instructions: Vec<&str>) -> i64 {
    let mut total: i64 = 0;
    let mut left_value: i64 = 0;
    let mut operation = 0;
    let mut current_index = 0;

    for x in instructions.clone() {
        print!("{} ", x);
    }
    println!();

    while current_index < instructions.len() {
        //println!("got {} at {}", instructions[current_index], current_index);
        if instructions[current_index].starts_with("(") {
            let mut parenthesis_ctr = 1;
            if instructions[current_index].starts_with("((") {
                parenthesis_ctr = 2;
            }
            for end_index in current_index + 1..instructions.len() {
                if instructions[end_index].starts_with("(") {
                    parenthesis_ctr += 1;
                }
                if instructions[end_index].ends_with(")") {
                    if instructions[end_index].ends_with("))") {
                        parenthesis_ctr -= 1;
                    }

                    if parenthesis_ctr == 1 {
                        //println!("counter was 1");
                        let curr: &str = &instructions[current_index][1..];
                        instructions[current_index] = curr;

                        let end: &str =
                            &instructions[end_index][0..instructions[end_index].len() - 1];
                        instructions[end_index] = end;
                        let result = calculateRecursivly(
                            instructions[current_index..end_index + 1].to_vec(),
                        );
                        //println!("result was {}, operation is {}", result, operation);
                        if operation > 0 {
                            if operation == 1 {
                                total = left_value + result;
                                //println!("total is now {}", total);
                                left_value = total;
                                operation = 0;
                            } else {
                                total = left_value * result;
                                //println!("total is now {}", total);
                                left_value = total;
                                operation = 0;
                            }
                        } else {
                            left_value = result;
                        }
                        current_index = end_index;
                        break;
                    } else {
                        //println!("counter was not 1, was {}", parenthesis_ctr);
                        parenthesis_ctr -= 1;
                    }
                }
            }
        } else {
            if instructions[current_index] == "+" {
                operation = 1;
            } else if instructions[current_index] == "*" {
                operation = 2;
            //right value
            // println!(
            //     "current index is {}, instruction len is {}",
            //     current_index,
            //     instructions.len()
            // );
            } else if operation > 0 {
                if operation == 1 {
                    total = left_value + instructions[current_index].parse::<i64>().unwrap();
                    //println!("total is now {}", total);
                    left_value = total;
                    operation = 0;
                } else {
                    total = left_value * instructions[current_index].parse::<i64>().unwrap();
                    //println!("total is now {}", total);
                    left_value = total;
                    operation = 0;
                }
            } else {
                left_value = instructions[current_index].parse::<i64>().unwrap();
            }
        }
        current_index += 1;
    }
    return total;
}

fn main() {
    // --snip--
    let filename = "src/Day18/test.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut result = 0;
    for line in lines {
        let instructions: Vec<&str> = line.split_whitespace().collect();
        let value = calculateRecursivly(instructions);
        println!("value was {}", value);
        result += value;
    }
    println!("result was {}", result);
}
