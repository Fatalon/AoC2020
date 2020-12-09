use std::fs;

fn estimateWrongValue(values: Vec<i64>) -> (usize, i64) {
    let preamble_length = 25; //TODO change to 25
    let mut preamble_start_index = 0;
    let mut preamble_end_index = preamble_length;

    for n in preamble_length..values.len() {
        let preamble = &values[preamble_start_index..preamble_end_index];
        let mut found_solution = false;
        for test_value in preamble {
            let result = values[n] - test_value;
            if preamble.iter().any(|&i| i == result) {
                found_solution = true;
                break;
            }
        }
        if found_solution {
            preamble_start_index += 1;
            preamble_end_index += 1;
            found_solution = false;
        } else {
            return (n, values[n]);
        }
    }
    return (0, 0);
}

fn main() {
    // --snip
    let filename = "src/Day9/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let values: Vec<i64> = contents.lines().map(|s| s.parse().unwrap()).collect();
    let result = estimateWrongValue(values.clone());
    println!("FOUND number {} in {} iteration", result.1, result.0);

    let mut found_solution = false;
    for start_sum in 0..result.0 {
        let mut total_sum = values[start_sum];
        //println!("iteration {} with value {}", start_sum, total_sum);
        for add in start_sum + 1..result.0 {
            total_sum += values[add];
            //println!("add is {}, sum is {}", add, total_sum);
            if total_sum == result.1 {
                found_solution = true;
                //println!("search now for solution in range {},{}", start_sum, add + 1);
                let vec_to_be_checked = &values[start_sum..add + 1];
                let min_value = vec_to_be_checked.iter().min();
                match min_value {
                    None => println!("Min value was not found"),
                    Some(i) => println!("Min Value = {}", i),
                }
                let max_value = vec_to_be_checked.iter().max();
                match max_value {
                    None => println!("Max value was not found"),
                    Some(i) => println!("Max Value = {}", i),
                }
                println!(
                    "final result is {}",
                    max_value.unwrap() + min_value.unwrap()
                )
            }
            if total_sum > result.1 {
                break;
            }
        }
    }
}
