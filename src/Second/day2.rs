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
        let char_value = char_string_splitted[1].chars().nth(0).unwrap();
        let min_max_splitted: Vec<usize> = char_string_splitted[0]
            .split("-")
            .map(|s| s.parse().unwrap())
            .collect();
        let fist_char_in_string = test_string.chars().nth(min_max_splitted[0] - 1).unwrap();
        let second_char_in_string = test_string.chars().nth(min_max_splitted[1] - 1).unwrap();
        // println!(
        //     "search for min: {} or max: {} of: {} in string {}",
        //     min_max_splitted[0], min_max_splitted[1], char_value, test_string
        // );
        if char_value == fist_char_in_string {
            if char_value != second_char_in_string {
                correct_values += 1;
                println!(
                    "search for min char: {} at {} or max char: {} at {} of: {} in string {}, was found exactly once at first : Correct",
                    fist_char_in_string,min_max_splitted[0],
                    second_char_in_string,min_max_splitted[1],
                    char_value,
                    test_string,
                );
            }
        } else {
            if char_value == second_char_in_string {
                correct_values += 1;
                println!(
                    "search for min char: {} at {} or max char: {} at {} of: {} in string {}, was found exactly once at second : Correct",
                    fist_char_in_string,min_max_splitted[0],
                    second_char_in_string,min_max_splitted[1],
                    char_value,
                    test_string,
                );
            }
        }
    }
    println!("Found {} correct entries", correct_values);
    //let mut splitted: Vec<(&str)> = strings.split(" : ");
    //.map(|s| s.parse().unwrap())
}
