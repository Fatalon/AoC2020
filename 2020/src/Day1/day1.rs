use std::env;
use std::fs;

fn main() {
    // --snip--
    let filename = "src/First/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    //println!("With text:\n{}", contents);
    let mut split: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();

    for num1 in &split {
        //println!("{}", num);
        if num1 < &1001 {
            println!("{} is smaller than 1001", num1);
            for num2 in &split {
                if num2 < &1001 {
                    for num3 in &split {
                        if num2 + num1 + num3 == 2020 {
                            println!(
                                "{}, {} and {} sum up to 2020, product is {}",
                                num1,
                                num2,
                                num3,
                                num1 * num2 * num3
                            );
                        }
                    }
                }
            }
        }
    }
}
