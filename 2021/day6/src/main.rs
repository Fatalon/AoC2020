use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut f = BufReader::new(File::open("input.txt").unwrap());
    let mut groups:Vec<u64> = vec![0,0,0,0,0,0,0,0,0];
    let mut line = String::new();
    f.read_line(&mut line).unwrap();
    line.split(',').for_each(|s| {
        let val = s.parse::<usize>().unwrap();
        groups[val] +=1;
    });

    for i in 0..257{
        println!("Cycle {} Current groups: {:?}!",i,groups);
        println!("Currently {} fishes alive", groups.iter().sum::<u64>());
        groups.rotate_left(1);
        groups[6] += groups[8];

    }
}
