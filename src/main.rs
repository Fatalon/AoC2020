use std::fs;

fn main() {
    // --snip
    let filename = "src/Day10/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut adapters: Vec<i64> = contents.lines().map(|s| s.parse().unwrap()).collect();

    adapters.sort();

    let mut last_value_in_array = *adapters.last().unwrap() + 3;
    adapters.push(last_value_in_array);
    println!("last value is {}", last_value_in_array);
    last_value_in_array += 1;

    let mut total_paths: Vec<i64> = vec![0; last_value_in_array as usize];
    total_paths[0] = 1;

    for adapter in adapters {
        total_paths[adapter as usize] = 0;
        for difference in adapter - 3..adapter {
            println!("test with {}", difference);
            if difference >= 0 {
                if total_paths[difference as usize] != 0 {
                    println!(
                        "the adapter is {}, the current values in total_paths are {}",
                        adapter, total_paths[difference as usize]
                    );
                    total_paths[adapter as usize] += total_paths[difference as usize];
                }
            }
        }
    }
    last_value_in_array -= 1;
    println!(
        "number of mutations is really {}",
        total_paths[last_value_in_array as usize]
    );
}
