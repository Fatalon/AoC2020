use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let mut f = BufReader::new(File::open("input.txt").unwrap());
    let mut s = String::new();
    f.read_line(&mut s).unwrap();
    let mut values: Vec<i32> = s.split(',').map(|s| s.parse().unwrap()).collect();

    println!("input :{:?}",values);
    values.sort();
    let mid = values.len()/2;
    let median = values[mid];
    println!("Median : {}",median);
    
    let total = values.iter().map(|x| (x-median).abs()).sum::<i32>();
    println!("total is {}",total);

    let mut crab_max = 0;
    for n in &values {
        crab_max = crab_max.max(*n);
    }

    let mut fuel_cost = vec![0; (crab_max+1) as usize];
    for crab_pos in &values {
        for i in 0..crab_max+1 {
            let n = i32::abs( i - crab_pos );
            fuel_cost[i as usize] += n*(n+1)/2;
        }
    }
    let final_total = fuel_cost.iter().min().unwrap();
    println!("total is {}",final_total);
}

// 101618985
// 101618069