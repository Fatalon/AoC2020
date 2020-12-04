use std::fs;

//use regex::Regex;

fn main() {
    // --snip--
    let filename = "src/Day4/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let passports: Vec<&str> = contents.split("\n\n").collect();

    let mut correct_passports: i32 = 0;

    for passport in passports {
        println!("Current passport: {}", passport);
        let fields_per_passport: Vec<&str> = passport.split_whitespace().collect();

        if fields_per_passport.len() == 8 {
            correct_passports += 1;
            println!("Correct amount of fields was found");
            continue;
        }
        if fields_per_passport.len() == 7 {
            let mut found_cid: bool = false;
            for field in fields_per_passport {
                let pair: Vec<&str> = field.split(":").collect();
                if pair[0] == "cid" {
                    found_cid = true;
                    break;
                }
            }
            if !found_cid {
                println!("was 7, but cid was missing, so it is counted as valid");
                correct_passports += 1;
            }
        }
    }
    println!("number of valid passports: {}", correct_passports);

    // println!(
    //     "first passport is: {}, number of passports is: {}",
    //     rows[0],
    //     rows.len()
    // );
    // let col = rows[0].len();
    // println!("number of rows: {}, elements per row: {}", rows.len(), col);
    // //println!("With text:\n{}", contents);
    // //let mut split: Vec<(&str, i32)> = contents
    // let mut counted_trees_1_1: i32 = 0;
    // let mut counted_trees_3_1: i32 = 0;
    // let mut counted_trees_5_1: i32 = 0;
    // let mut counted_trees_7_1: i32 = 0;
    // let mut counted_trees_1_2: i32 = 0;

    // let mut current_row_index: usize = 0;

    // let mut current_col_index_1: usize = 0;
    // let mut current_col_index_3: usize = 0;
    // let mut current_col_index_5: usize = 0;
    // let mut current_col_index_7: usize = 0;

    // let mut current_col_index_1_doubled: usize = 0;

    // for x in rows {
    //     let char_value_1 = x.chars().nth(current_col_index_1).unwrap();
    //     let char_value_3 = x.chars().nth(current_col_index_3).unwrap();
    //     let char_value_5 = x.chars().nth(current_col_index_5).unwrap();
    //     let char_value_7 = x.chars().nth(current_col_index_7).unwrap();

    //     if char_value_1 == '#' {
    //         counted_trees_1_1 += 1;
    //     }
    //     if char_value_3 == '#' {
    //         counted_trees_3_1 += 1;
    //     }
    //     if char_value_5 == '#' {
    //         counted_trees_5_1 += 1;
    //     }
    //     if char_value_7 == '#' {
    //         counted_trees_7_1 += 1;
    //     }

    //     if current_row_index % 2 == 0 {
    //         let char_value_1_doubled = x.chars().nth(current_col_index_1_doubled).unwrap();
    //         if char_value_1_doubled == '#' {
    //             counted_trees_1_2 += 1;
    //         }
    //         current_col_index_1_doubled = (current_col_index_1_doubled + 1) % col;
    //     }

    //     current_row_index += 1;

    //     current_col_index_1 = (current_col_index_1 + 1) % col;
    //     current_col_index_3 = (current_col_index_3 + 3) % col;
    //     current_col_index_5 = (current_col_index_5 + 5) % col;
    //     current_col_index_7 = (current_col_index_7 + 7) % col;
    //     // println!(
    //     //     "search for min: {} or max: {} of: {} in string {}",
    //     //     min_max_splitted[0], min_max_splitted[1], char_value, test_string
    //     // );
    // }
    // println!("Found {} trees in 1x1", counted_trees_1_1);
    // println!("Found {} trees in 3x1", counted_trees_3_1);
    // println!("Found {} trees in 5x1", counted_trees_5_1);
    // println!("Found {} trees in 7x1", counted_trees_7_1);
    // println!("Found {} trees in 1x2", counted_trees_1_2);
    // let result: i128 = counted_trees_1_1 as i128
    //     * counted_trees_3_1 as i128
    //     * counted_trees_5_1 as i128
    //     * counted_trees_7_1 as i128
    //     * counted_trees_1_2 as i128;
    // println!("Result is {}", result);
}
