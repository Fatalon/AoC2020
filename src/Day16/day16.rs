use std::collections::HashMap;
use std::fs;

fn recursivly_assign(
    mut unassigned_rules: Vec<usize>,
    mut not_matching_rules: Vec<Vec<usize>>,
    mut assigned_columns: Vec<usize>,
) -> Vec<(usize, usize)> {
    let mut found_i = 100;
    let mut found_index = 100;
    for (i, column) in not_matching_rules.iter().enumerate() {
        //find the one that is missing
        if assigned_columns.iter().any(|&f| f == i) {
            //println!("{} was already assigned", i);
            continue;
        }
        if column.len() == unassigned_rules.len() - 1 {
            //estimate the rule
            for (index, unassigned_rule) in unassigned_rules.iter().enumerate() {
                if column.iter().any(|&i| i == *unassigned_rule) {
                    continue;
                }
                //println!("rule {} was not found in column {}", unassigned_rule, i);
                if column.len() == 0 {
                    return vec![(*unassigned_rule, i)];
                } else {
                    found_i = i;
                    found_index = index;
                    break;
                }
            }
        }
    }
    let unassigned_rule = unassigned_rules[found_index];
    unassigned_rules.remove(found_index);
    // println!(
    //     "length of unassigned rules is now {}",
    //     unassigned_rules.len()
    // );
    assigned_columns.push(found_i);
    let mut result = recursivly_assign(unassigned_rules, not_matching_rules, assigned_columns);
    result.push((unassigned_rule, found_i));
    return result;
}

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

    for x in reference_table.clone() {
        println!("rule {} at index {}", x.1, x.0);
    }
    for (i, x) in ranges.iter().enumerate() {
        print!("value {} can be of types:", i);
        for value in x {
            print!(" {}, ", value)
        }
        println!();
    }

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
    let mut invalid_tickets: Vec<usize> = vec![];

    for (i, ticket) in nearby_ticket_values.iter().enumerate() {
        for value in ticket.clone() {
            if ranges[value].is_empty() {
                invalid_values.push(value);
                invalid_tickets.push(i);
            }
        }
    }

    //part 2, solve the puzzle
    //restructure nearby tickets, now vector of valid entries per rule
    let mut nearby_values_per_column: Vec<Vec<usize>> = vec![vec![]; reference_table.len()];
    for (i, ticket) in nearby_ticket_values.iter().enumerate() {
        if invalid_tickets.iter().any(|&entry| entry == i) {
            println!("ticket {} was invalid, continue", i);
            continue;
        }
        for (number, value) in ticket.iter().enumerate() {
            let mut curr = nearby_values_per_column[number].clone();
            curr.push(*value);
            nearby_values_per_column[number] = curr;
        }
    }

    for (i, x) in nearby_values_per_column.iter().enumerate() {
        print!("index {} has values:", i);
        for value in x {
            print!(" {}, ", value)
        }
        println!();
    }

    let mut not_matching_rules: Vec<Vec<usize>> = vec![vec![]; reference_table.len()];

    for (i, values) in nearby_values_per_column.iter().enumerate() {
        //all values need to fulfill the same rule -> make an and -> if one value is not compliant, continue with next rule
        for rule in 0..reference_table.len() {
            for value in values.clone() {
                if ranges[value].iter().any(|&i| i == rule) {
                    continue;
                }
                //was not rule compliant
                let mut curr = not_matching_rules[i].clone();
                curr.push(rule);
                not_matching_rules[i] = curr;
            }
        }
    }
    let mut unassigned_rules: Vec<usize> = vec![];
    for x in reference_table.clone() {
        unassigned_rules.push(x.0);
    }
    for (i, x) in not_matching_rules.iter().enumerate() {
        print!("column {} does not match rules:", i);
        for value in x {
            print!(" {}, ", value)
        }
        println!();
    }
    let result = recursivly_assign(unassigned_rules, not_matching_rules, vec![]);

    let mut part2: i128 = 1;
    for x in result {
        println!("rule {} is assigned to column {}", x.0, x.1);
        let description = reference_table.get(&x.0).unwrap();
        if description.contains("departure") {
            println!("description is {} at index {}", description, x.1);
            part2 *= my_ticket_values[x.1] as i128;
        }
    }

    println!("Solution to part 2 is {}", part2);

    let mut total_sum = 0;
    for x in invalid_values {
        total_sum += x;
    }
    println!("solution is {}", total_sum);
}
