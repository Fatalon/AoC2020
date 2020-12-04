use std::fs;

use regex::Regex;

fn validate_fields(fields: Vec<&str>) -> bool {
    for field in fields {
        let pair: Vec<&str> = field.split(":").collect();
        if pair[0] == "byr" {
            let year: i32 = pair[1].parse().unwrap();
            if year > 2002 || year < 1920 {
                println!("Accused wrong because of {}, {}", pair[0], pair[1]);
                return false;
            }
            continue;
        }
        if pair[0] == "iyr" {
            let year: i32 = pair[1].parse().unwrap();
            if year > 2020 || year < 2010 {
                println!("Accused wrong because of {}, {}", pair[0], pair[1]);
                return false;
            }
            continue;
        }
        if pair[0] == "eyr" {
            let year: i32 = pair[1].parse().unwrap();
            if year > 2030 || year < 2020 {
                println!("Accused wrong because of {}, {}", pair[0], pair[1]);
                return false;
            }
            continue;
        }
        if pair[0] == "hgt" {
            if pair[1].contains("cm") {
                let height: i32 = pair[1].replace("cm", "").parse().unwrap();
                if height > 193 || height < 150 {
                    println!("Accused wrong because of {}, {}", pair[0], pair[1]);
                    return false;
                }
                continue;
            } else if pair[1].contains("in") {
                let height: i32 = pair[1].replace("in", "").parse().unwrap();
                if height > 76 || height < 59 {
                    println!("Accused wrong because of {}, {}", pair[0], pair[1]);
                    return false;
                }
                continue;
            }
            return false;
        }
        if pair[0] == "hcl" {
            let hcl_re = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
            //let v: Vec<&str> = pair[1].matches(&hcl_re).collect();
            if !hcl_re.is_match(pair[1]) {
                println!("Accused wrong because of {}, {}", pair[0], pair[1]);
                return false;
            }
            continue;
        }
        if pair[0] == "ecl" {
            let matches: Vec<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if !matches.contains(&pair[1]) {
                println!("Accused wrong because of {}, {}", pair[0], pair[1]);
                return false;
            }
            continue;
        }
        if pair[0] == "pid" {
            let pid_re = Regex::new(r"^(\d){9}$").unwrap();
            if !pid_re.is_match(pair[1]) {
                println!("Accused wrong because of {}, {}", pair[0], pair[1]);
                return false;
            }
            continue;
        }
    }
    true
}

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

        let count: usize = fields_per_passport.len();

        if count == 8 {
            if validate_fields(fields_per_passport.clone()) {
                correct_passports += 1;
                println!("Correct amount of fields was found");
                continue;
            }
        }
        if count == 7 {
            let mut found_cid: bool = false;
            for field in fields_per_passport.clone() {
                let pair: Vec<&str> = field.split(":").collect();
                if pair[0] == "cid" {
                    found_cid = true;
                    break;
                }
            }
            if !found_cid {
                if validate_fields(fields_per_passport.clone()) {
                    correct_passports += 1;
                    println!("was 7, but cid was missing, so it is counted as valid");
                    continue;
                }
            }
        }
    }
    println!("number of valid passports: {}", correct_passports);
}
