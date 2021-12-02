use std::fs;

fn task1(lines: &Vec<i32>){
    let mut total:i32 = 0;
    for (pos, e) in lines.iter().enumerate() {
        if pos == 0{
            continue;
        }
        if *e > lines[pos-1]{
            total +=1;
        }
        println!("Element at position {}: {:?}", pos, e);
    }
    println!("Total was {}",total);
}

fn task2(lines: &Vec<i32>){
    let mut total:i32 = 0;
    for (pos, e) in lines.iter().enumerate() {
        //skip first 3 elements
        if pos < 3{
            continue;
        }
        let total_low_window:i32 = lines[pos-1] + lines[pos-2] + lines[pos-3];
        let total_high_window:i32 = *e + lines[pos-1] + lines[pos-2];
        if total_high_window > total_low_window{
            total +=1;
        }
        println!("Element at position {}: {:?}", pos, e);
    }
    println!("Total was {}",total);
}

fn main() {
    // --snip--
    let filename = "src/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();
    task1(&split);
    task2(&split);
}
