use std::fs;

fn main() {
    // --snip--
    let filename = "src/Second/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let strings: Vec<&str> = contents.lines().collect();
    //println!("With text:\n{}", contents);
    //let mut split: Vec<(&str, i32)> = contents
    let mut correct_values: i32 = 0;
    for x in strings {
        let test_string_splitted: Vec<&str> = x.split(": ").collect();
        let test_string = test_string_splitted[1];
        let char_string_splitted: Vec<&str> = test_string_splitted[0].split_whitespace().collect();
        let char_value = char_string_splitted[1];
        let min_max_splitted: Vec<usize> = char_string_splitted[0]
            .split("-")
            .map(|s| s.parse().unwrap())
            .collect();

        // println!(
        //     "search for min: {} or max: {} of: {} in string {}",
        //     min_max_splitted[0], min_max_splitted[1], char_value, test_string
        // );
        let counted_occurences = test_string.matches(char_value).count();
        if counted_occurences <= min_max_splitted[1] && counted_occurences >= min_max_splitted[0] {
            correct_values += 1;
            println!(
                "search for min: {} or max: {} of: {} in string {}, was found: {} : Correct",
                min_max_splitted[0],
                min_max_splitted[1],
                char_value,
                test_string,
                counted_occurences
            );
        }
    }
    println!("Found {} correct entries", correct_values);
    //let mut splitted: Vec<(&str)> = strings.split(" : ");
    //.map(|s| s.parse().unwrap())
}
