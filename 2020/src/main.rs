use std::convert::From;
use std::fs;

use itertools::enumerate;
use std::collections::HashMap;

fn main() {
    // --snip--
    let filename = "src/Day19/test.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut splits = contents.split("\n\n");
    let mut rules: Vec<&str> = splits.next().unwrap().lines().collect();
    let rules: Vec<Rule> = rules.into_iter().map(|s| Rule::from(s)).collect();

    let signals: Vec<&str> = splits.next().unwrap().lines().collect();
    let mut rules_map = HashMap::new();
    for rule in rules.clone() {
        rules_map.insert(rule.id, rule);
    }

    println!("found rules {:?}", rules_map);
    println!("found signals {:?}", signals);

    let mut rules_to_Strings = HashMap::new();
    for rule in rules.clone() {
        if rule.resolved {
            let cont: Vec<Vec<String>> = vec![vec![rule.content[0]]];
            rules_to_Strings.insert(rule.id, generate_strings(cont));
        }
    }

    println!("rules to String is currently {:?}", rules_to_Strings);
    for _ in 1..100 {
        for mut rule in rules.clone() {
            if rule.resolved {
                continue;
            }
            let mut new_cont:Vec<Vec<String>>;
            for (i, nested_id) in enumerate(rule.content) {
                if nested_id == "|" {
                    continue;
                }
                let nested_id: u16 = nested_id.parse().unwrap();
                let nested_rule = rules_to_Strings.get(&nested_id);
                if nested_rule.is_some() {
                    new_cont[i] = *nested_rule.unwrap();
                    
                    // new_cont[i] = substring;
                    // if substring.contains("|") {
                    //     let splits = substring.split("|");
                    //     for (index,split) in enumerate(splits.into_iter()){
                    //         insert_in_place(new_cont.as_mut(), split.to_owned(), i+index);
                    //         new_cont.push(split.to_owned())
                    //     }
                    // } else {
                    // }
                }else{
                    // one value was not substitutable, continue
                    break;
                }
            }
            rule.content = generate_strings(new_cont);
            rule.resolved = true;

            rules_to_Strings.insert(rule.id, rule.content);
        }
    }

    println!("Updated rules {:?}", rules_map);
}

#[derive(Debug, Clone)]
struct Rule {
    id: u16,
    content: Vec<String>,
    resolved: bool,
}

impl<'a> From<&'a str> for Rule {
    fn from(item: &'a str) -> Self {
        let mut splits = item.split(":");
        let id: u16 = match splits.next() {
            Some(x) => x.parse().unwrap(),
            None => 999,
        };
        let mut resolved: bool = false;
        let cont: Vec<String> = match splits.next() {
            Some(x) => {
                let mut pattern: Vec<String> = x.split(" ").skip(1).map(|s| s.to_owned()).collect();
                if pattern[0].starts_with("\"") {
                    pattern[0] = pattern[0]
                        .split("\"")
                        .skip(1)
                        .next()
                        .map(|s| s.to_owned())
                        .unwrap();
                    resolved = true;
                    pattern
                } else {
                    resolved = false;
                    pattern
                }
            }
            None => Vec::new(),
        };
        Rule {
            id: (id),
            content: (cont),
            resolved: (resolved),
        }
    }
}

fn insert_in_place<T>(array: &mut Vec<T>, value: T, index: usize) {
    array[index..].rotate_right(1);
    array[index] = value;
}

fn generate_strings(rule: Vec<Vec<String>>) -> Vec<String> {
    let mut return_value = Vec::new();
    let mut last_index:usize = 0;
    for (i,part) in enumerate(rule.clone()){
        for strings in part {
            if strings == "|" {
                for mut first in rule.get(last_index).unwrap().to_owned(){
                    for second in rule.get(i-1).unwrap(){
                        first.push_str(second.as_str());
                        return_value.push(first);
                    }
                }
            } 
            last_index = i+1;
        }
    }
    return_value
}
