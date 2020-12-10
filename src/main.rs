use std::fs;

fn main() {
    // --snip
    let filename = "src/Day10/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut adapters: Vec<i64> = contents.lines().map(|s| s.parse().unwrap()).collect();

    adapters.sort();

    let mut diff_by_1 = 0;
    let mut diff_by_2 = 0;
    let mut diff_by_3 = 1; //for the phones own adapter

    let mut last_value = 0;

    for adapter in adapters {
        let diff = adapter - last_value;
        last_value = adapter;

        if diff == 1 {
            diff_by_1 += 1;
            continue;
        }
        if diff == 2 {
            diff_by_2 += 1;
            continue;
        }
        if diff == 3 {
            diff_by_3 += 1;
            continue;
        }
        println!(
            "Found suspecious values, was {}, with diff {}",
            adapter, diff
        );
    }
    println!(
        "Found {} diff 1 and {} diff 3, product is {}",
        diff_by_1,
        diff_by_3,
        diff_by_1 * diff_by_3
    );
}
