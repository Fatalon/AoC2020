use std::fs;

fn main() {
    // --snip
    let filename = "src/Day9/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let values: Vec<i64> = contents.lines().map(|s| s.parse().unwrap()).collect();

    let preamble_length = 25; //TODO change to 25
    let mut preamble_start_index = 0;
    let mut preamble_end_index = preamble_length;

    for n in preamble_length..values.len() {
        println!("{} th value was {}", n, values[n]);
        println!(
            "will test in range {}, to {}, which are {} and {}",
            preamble_start_index,
            preamble_end_index,
            values[preamble_start_index],
            values[preamble_end_index]
        );
        let preamble = &values[preamble_start_index..preamble_end_index];
        let mut found_solution = false;
        for test_value in preamble {
            let result = values[n] - test_value;
            if preamble.iter().any(|&i| i == result) {
                println!(
                    "found valid solution for {} th value {} with {} and {}",
                    n, values[n], test_value, result
                );
                found_solution = true;
                break;
            }
        }
        if found_solution {
            preamble_start_index += 1;
            preamble_end_index += 1;
            found_solution = false;
        } else {
            println!("FOUND number {} in {} iteration", values[n], n);
            break;
        }
    }
}
