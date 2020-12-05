use std::fs;

fn main() {
    // --snip--
    let filename = "src/Day5/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let seats: Vec<&str> = contents.lines().collect();

    let mut highest_seat_id: isize = 0;

    for seat in seats {
        println!("Seat is {}", seat);
        let row = &seat.replace("B", "1").replace("F", "0")[..7];
        println!("row in binary is: {}", row);
        let row = isize::from_str_radix(row, 2).unwrap();
        println!("row is: {}", row);

        let col = &seat.replace("L", "0").replace("R", "1")[7..10];
        println!("col in binary is: {}", col);
        let col = isize::from_str_radix(col, 2).unwrap();
        println!("col is: {}", col);

        let seat_id = row * 8 + col;
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }
    println!("highest seat id was: {}", highest_seat_id);
}
