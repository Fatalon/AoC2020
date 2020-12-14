use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn generatePossibleAddresses(
    mut floating_indices: Vec<usize>,
    bin_address: Vec<char>,
) -> Vec<Vec<char>> {
    if floating_indices.len() == 0 {
        return vec![bin_address];
    }
    let index = floating_indices.pop().unwrap();
    let mut result = generatePossibleAddresses(floating_indices, bin_address);
    let copy = result.clone();
    for mut entry in copy {
        entry[index as usize] = '1';
        result.push(entry.clone());
        entry[index as usize] = '0';
        result.push(entry.clone());
    }
    return result;
}

fn main() {
    // --snip
    let filename = "src/Day14/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut memory = HashMap::new();

    let mut mask = "";

    for line in lines {
        let instruction: Vec<&str> = line.split(" = ").collect();
        if instruction[0] == "mask" {
            mask = instruction[1];
            println!("Mask is now \t\t  {}", mask);
        } else {
            let raw_int_address: i64 = instruction[0]
                .replace("mem[", "")
                .replace("]", "")
                .parse()
                .unwrap();
            let raw_int_value: i64 = instruction[1].parse().unwrap();

            println!("instruction put {} in {}", raw_int_value, raw_int_address);

            let raw_bin_address = format!("{:#038b}", raw_int_address);
            println!("bin value is \t\t{}", raw_bin_address);
            let mut bin_address: Vec<_> = raw_bin_address.chars().collect();
            bin_address.reverse();
            let mut mask_vec: Vec<_> = mask.chars().collect();
            mask_vec.reverse();

            let mut floating_indices = Vec::new();
            for n in 0..mask_vec.len() {
                let c = mask_vec[n];
                if c == '1' {
                    bin_address[n] = c;
                }
                if c == 'X' {
                    floating_indices.push(n);
                }
            }
            let variations = i64::pow(2, floating_indices.len() as u32);
            println!("need {} different addresses", variations);

            let mut result = generatePossibleAddresses(floating_indices, bin_address);

            let set: HashSet<_> = result.drain(..).collect(); // dedup
            result.extend(set.into_iter());
            println!("result had {} entries", result.len());

            for mut entry in result {
                entry.reverse();
                let bin_idx: String = entry.iter().collect();
                let bin_idx = &bin_idx[2..];
                println!("bin is now \t\t  {}", bin_idx);
                let int_address = i64::from_str_radix(&*bin_idx, 2).unwrap();
                println!("insert {} into address {}", raw_int_value, int_address);
                memory.insert(int_address, raw_int_value);
            }
        }
    }
    let mut sum = 0;
    for entry in memory {
        sum += entry.1;
    }
    println!("total sum is {}", sum);
}
