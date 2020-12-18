use std::fs;
fn main() {
    // --snip--
    let filename = "src/Day17/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();
}
