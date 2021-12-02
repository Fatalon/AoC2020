use modinverse;
use std::fs;

fn input_generator(input: &str) -> (u128, Vec<Option<u128>>) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse().unwrap(),
        lines
            .next()
            .unwrap()
            .split(',')
            .map(|l| l.parse().ok())
            .collect(),
    )
}

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
    let filename = "src/Day13/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let result = input_generator(&contents);
    let res = part2(&result);
    println!("res is {}", res);
}
