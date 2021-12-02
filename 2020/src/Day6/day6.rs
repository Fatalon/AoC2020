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
        let mut current_answers = HashSet::new();
        let answers: Vec<&str> = group.lines().collect();
        let first_answer = answers[0];

        //init
        for c in first_answer.chars() {
            total_answers.insert(c);
        }

        //intesect
        for answer in answers {
            current_answers.clear();
            for c in answer.chars() {
                current_answers.insert(c);
            }
            total_answers = total_answers
                .iter()
                .filter(|k| current_answers.contains(k))
                .map(|&x| x)
                .collect::<HashSet<char>>();
            //total_answers.intersection(&current_answers);
        }
        println!(
            "Found {} same answers in {}",
            total_answers.len(),
            group.replace("\n", "\t")
        );
        total_score += total_answers.len();
    }
    println!("total score was: {}", total_score);
}
