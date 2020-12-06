use std::collections::HashSet;
use std::fs;

fn main() {
    // --snip--
    let filename = "src/Day6/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let groups: Vec<&str> = contents.split("\n\n").collect();

    let mut total_score: usize = 0;

    for group in groups {
        let mut total_answers = HashSet::new();
        let answers: Vec<&str> = group.lines().collect();
        for answer in answers {
            for c in answer.chars() {
                total_answers.insert(c);
                // do something with `c`
            }
            // do something with `c`
        }
        println!(
            "Found {} disjunct answers in {}",
            total_answers.len(),
            group.replace("\n", "\t")
        );
        total_score += total_answers.len();
    }
    println!("total score was: {}", total_score);
}
