use std::fs;

fn estimateNearest(buses: Vec<i32>, time_of_arrival: i32) -> i32 {
    let mut nearest_value = 10000;
    let mut nearest_bus_id = 0;
    for bus in buses {
        if time_of_arrival % bus == 0 {
            nearest_value = 0;
            nearest_bus_id = 0;
            return 0;
        }
        let amount_of_departs = time_of_arrival / bus;
        let next_depart = (amount_of_departs + 1) * bus;
        if (next_depart - time_of_arrival) < nearest_value {
            nearest_bus_id = bus;
            nearest_value = next_depart - time_of_arrival;
        }
    }
    return nearest_value * nearest_bus_id;
}

fn estimateOptimalTimeForArrival(buses_with_offset: Vec<(i32, i32)>) {
    let mut time: i128 = 0;
    let mut all_valid = true;
    for _ in 0..10000000000000000 as i128 {
        time += buses_with_offset[0].1 as i128;
        all_valid = true;
        if time % 500000000 == 0 {
            println!("time is now {}", time);
        }
        for entry in buses_with_offset.clone() {
            if (time + entry.0 as i128) % entry.1 as i128 != 0 {
                // println!(
                //     "entry failed for time {} with values {},{}",
                //     time, entry.0, entry.1
                // );
                all_valid = false;
                break;
            }
        }
        if all_valid {
            println!("found solution at time {}", time);
            return;
        }
    }
}

fn main() {
    // --snip
    let filename = "src/Day13/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let time_of_arrival = lines[0].parse().unwrap();

    let mut buses: Vec<i32> = Vec::new();
    for entry in lines[1].split(",") {
        if entry == "x" {
            continue;
        }
        buses.push(entry.parse().unwrap());
    }

    for bus in buses.clone() {
        print!("found {}, ", bus);
    }
    println!("Search for time {}", time_of_arrival);

    let result = estimateNearest(buses.clone(), time_of_arrival);
    println!("result is {}", result);

    let mut buses_with_offset: Vec<(usize, usize)> = Vec::new();

    let buses = lines[1]
        .split(',')
        .enumerate()
        .filter(|&(_, bus_id)| bus_id != "x")
        .map(|(i, bus_id)| (i, bus_id.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();

    for bus in buses.clone() {
        println!("bus with id {} should depart at {}", bus.1, bus.0);
    }

    //estimateOptimalTimeForArrival(buses_with_offset.clone());
}
