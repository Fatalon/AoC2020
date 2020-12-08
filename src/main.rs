use std::fs;

fn how_many_bags_needs(name: &str, rules: &Vec<&str>) -> i32 {
    println!("Got name {}, len of rules is {}", name, rules.len());
    for rule in rules {
        let items: Vec<&str> = rule.split("s contain ").collect();
        if rule.len() == 0 {
            println!("found empty last rule");
            continue;
        }
        if !items[0].contains(name) {
            continue;
        }

        if items[1].contains("no other bags") {
            println!("found special case");
            return 1;
        }
        let contained_items: Vec<&str> = items[1].split(", ").collect();

        let mut total_sum: i32 = 0;
        println!(
            "found item {} to be {}, contains: {}",
            items[0], name, items[1]
        );
        for contained_item in contained_items {
            let amounts: Vec<&str> = contained_item.split(" ").collect();
            let amount: i32 = amounts[0].parse().unwrap();
            let new_search_name = contained_item.chars().skip(2).collect::<String>();
            let string_len = new_search_name.len() - 2;
            let new_search_name = &new_search_name[..string_len];
            total_sum += amount * how_many_bags_needs(new_search_name, rules);
            println!("total sum is right now {}", total_sum);
        }
        return total_sum + 1;
    }
    return 0;
}

fn main() {
    // --snip
    let filename = "src/Day7/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut rules: Vec<&str> = contents.split(".\n").collect();

    let mut start_string = "shiny gold bag";
    let result = how_many_bags_needs(start_string, &rules);

    println!("total bugs in shiny gold was: {}", result - 1);
}
