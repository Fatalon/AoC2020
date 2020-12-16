use std::collections::HashMap;
use std::fs;

fn main() {
    // --snip--
    let filename = "src/Day16/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let rules: Vec<&str> = parts[0].lines().collect();

    let mut reference_table = HashMap::new();
    let mut ranges: Vec<Vec<_>> = vec![vec![]; 1000];

    for (i, rule) in rules.iter().enumerate() {
        let name_of_rule: Vec<&str> = rule.split(": ").collect();
        reference_table.insert(i, name_of_rule[0]);

        let unparsed_ranges: Vec<&str> = name_of_rule[1].split(" or ").collect();

        for range in unparsed_ranges {
            let values: Vec<usize> = range.split("-").map(|s| s.parse().unwrap()).collect();
            for value in values[0]..values[1] + 1 {
                let mut curr = ranges[value].clone();
                curr.push(i);
                ranges[value] = curr;
            }
        }
    }

    for x in reference_table {
        println!("rule {} at index {}", x.1, x.0);
    }
    // for (i, x) in ranges.iter().enumerate() {
    //     print!("value {} can be of types:", i);
    //     for value in x {
    //         print!(" {}, ", value)
    //     }
    //     println!();
    // }

    let my_ticket_values: Vec<&str> = parts[1].lines().collect();
    let my_ticket_values: Vec<usize> = my_ticket_values[1]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let nearby_values: Vec<&str> = parts[2].lines().collect();
    let mut nearby_ticket_values: Vec<Vec<usize>> = vec![];
    for (i, line) in nearby_values.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let parsed_line: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
        nearby_ticket_values.push(parsed_line);
    }

    let mut invalid_values: Vec<usize> = vec![];

    for ticket in nearby_ticket_values {
        for value in ticket {
            if ranges[value].is_empty() {
                invalid_values.push(value);
            }
        }
    }

    let mut total_sum = 0;
    for x in invalid_values {
        total_sum += x;
    }
    println!("solution is {}", total_sum);
}
