use std::collections::HashMap;
use std::fs;

fn part2(&(_, ref buses): &(u128, Vec<Option<u128>>)) -> u128 {
    //constraints in same format as before (id,time)
    let constraints = buses
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| b.map(|b| (b, i as u128)))
        .collect::<Vec<_>>();

    for i in constraints.clone() {
        println!("constraints are {} {}", i.0, i.1);
    }

    //prod over all bus ids
    let prod: u128 = constraints.iter().map(|&(b, _)| b).product();
    println!("prod is \t\t{}", prod);
    let factors: u128 = constraints.iter().map(|&(b, i)| i * prod / b).sum::<u128>() % prod;
    println!("factor is \t\t{}", factors);
    let factors = prod - factors;
    println!("factor is now \t\t{}", factors);
    let sum: u128 = constraints.iter().map(|&(b, _)| prod / b).sum();
    println!("sum is \t\t\t{}", sum);
    let isum = modinverse::modinverse(sum as i128, prod as i128).unwrap() as u128;
    (isum * factors) % prod
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
            println!("Mask is now {}", mask);
        } else {
            let address: i64 = instruction[0]
                .replace("mem[", "")
                .replace("]", "")
                .parse()
                .unwrap();
            let raw_int_value: i64 = instruction[1].parse().unwrap();
            println!("instruction put {} in {}", raw_int_value, address);
            let raw_bin_value = format!("{:#038b}", raw_int_value);
            println!("bin value is {}", raw_bin_value);
            let mut bin_value: Vec<_> = raw_bin_value.chars().collect();
            bin_value.reverse();
            let mut mask_vec: Vec<_> = mask.chars().collect();
            mask_vec.reverse();
            for n in 0..mask_vec.len() {
                println!("compare {} with {}", mask_vec[n], bin_value[n]);
                let c = mask_vec[n];
                if c != 'X' {
                    bin_value[n] = c;
                }
            }
            bin_value.reverse();
            let mut bin_idx: String = bin_value.iter().collect();
            let bin_idx = &bin_idx[2..];
            println!("bin is now {}", bin_idx);
            let intval = i64::from_str_radix(&*bin_idx, 2).unwrap();
            println!("insert {} into address {}", intval, address);
            memory.insert(address, intval);
        }
    }
    let mut sum = 0;
    for entry in memory {
        sum += entry.1;
    }
    println!("total sum is {}", sum);
}
