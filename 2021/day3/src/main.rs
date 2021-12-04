use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> Vec<Vec<u32>> {
    let f = BufReader::new(File::open("src/test.txt").unwrap());
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    let arguments: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(2).unwrap()).collect())
        .collect();
    // for x in &arguments {
    //     for y in x {
    //         print!("{}", y);
    //     }
    //     println!();
    // }
    return arguments;
}

fn task1() {
    let arguments: Vec<Vec<u32>> = read_file();
    let mut epsilon: String = "".to_string();
    let mut gamma: String = "".to_string();
    let y_max = arguments[0].len();
    let x_max = arguments.len();
    println!("{}, {}", x_max, y_max);
    for y in 0..y_max {
        let mut ones = 0;
        for x in 0..x_max {
            if arguments[x][y] == 1 {
                ones += 1;
            } else {
                ones -= 1;
            }
        }
        if ones > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    println!("results are {}, {}", gamma, epsilon);
    let gamma_decimal = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_decimal = isize::from_str_radix(&epsilon, 2).unwrap();
    println!(
        "task 1, result was {} times {} equals {}",
        gamma_decimal,
        epsilon_decimal,
        gamma_decimal * epsilon_decimal
    );
}

fn task2() {
    let ls: Vec<Vec<char>> = BufReader::new(File::open("src/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut nums_oxy = ls.clone();

    for bit in 0..ls[0].len() {
        let bits = nums_oxy.iter().map(|x| x[bit]).collect::<Vec<_>>();

        let c0 = bits.iter().filter(|x| **x == '0').count();
        let c1 = bits.iter().filter(|x| **x == '1').count();

        let keep = if c1 >= c0 { '1' } else { '0' };

        nums_oxy.retain(|x| x[bit] == keep);
        if nums_oxy.len() == 1 {
            break;
        }
    }

    let mut nums_car = ls.clone();

    for bit in 0..ls[0].len() {
        let bits = nums_car.iter().map(|x| x[bit]).collect::<Vec<_>>();

        let c0 = bits.iter().filter(|x| **x == '0').count();
        let c1 = bits.iter().filter(|x| **x == '1').count();

        let keep = if c1 >= c0 { '0' } else { '1' };

        nums_car.retain(|x| x[bit] == keep);
        if nums_car.len() == 1 {
            break;
        }
    }

    let oxy = u32::from_str_radix(&nums_oxy[0].iter().collect::<String>(), 2).unwrap();
    let car = u32::from_str_radix(&nums_car[0].iter().collect::<String>(), 2).unwrap();

    println!("part 2 : {:?}", oxy * car);
}

fn main() {
    task1();
    task2();
}
