use std::collections::HashSet;
use std::fs;

fn main() {
    // --snip
    let filename = "src/Day7/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut rules: Vec<&str> = contents.split(".\n").collect();

    let mut found = HashSet::new();
    found.insert("shiny gold");
    let mut total_options: usize = 0;

    while total_options != found.len() {
        total_options = found.len();
        for rule in &rules {
            let items: Vec<&str> = rule.split("s contain ").collect();
            if rule.len() == 0 {
                println!("found empty last rule");
                continue;
            }
            println!("rule was {}", rule);
            println!("rule for bag {} was {}", items[0], items[1]);

            if items[1].contains("no other bags") {
                println!("found special case");
                continue;
            }
            let contained_items: Vec<&str> = items[1].split(", ").collect();

            for contained_item in contained_items {
                let amounts: Vec<&str> = contained_item.split(" ").collect();
                let amount: i32 = amounts[0].parse().unwrap();
                if found.iter().any(|&i| contained_item.contains(i)) {
                    println!("Found one already present entry, push new one in");
                    found.insert(items[0]);
                }
            }
        }
    }
    println!("total options for shiny gold was: {}", found.len() - 1);
}
