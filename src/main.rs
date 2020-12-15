use std::fs;

fn main() {
    // --snip
    let filename = "src/Day15/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let values: Vec<i32> = contents.split(",").map(|s| s.parse().unwrap()).collect();

    let mut game: Vec<(i32, usize)> = Vec::new();

    for (i, value) in values.iter().enumerate() {
        game.push((*value, i + 1));
    }
    println!("game has {} entries", game.len());

    let mut last_value = -1;
    for index in values.len()..30000000 {
        if index % 10000 == 0 {
            println!("current index {}", index);
        }
        //println!("last value is {}", last_value);
        //println!("Current game state:");
        // for entry in game.clone() {
        //     println!("{} {}", entry.0, entry.1);
        // }
        if last_value == -1 {
            game.push((0, index + 1));
            last_value = 0;
            //println!("First iteration, index is currently {}", index);
            continue;
        }
        let mut game_to_check: Vec<(i32, usize)> = game.clone()[0..game.len() - 1].to_vec();
        game_to_check.reverse();
        //println!("game to check has length {}", game_to_check.len());
        // for entry in game_to_check.clone() {
        //     println!("{} {}", entry.0, entry.1);
        // }

        let mut last_time_seen = 0;
        for (i, entry) in game_to_check.iter().enumerate() {
            if entry.0 == last_value {
                // println!("found last value in entry {}, {}", entry.0, entry.1);
                last_time_seen = entry.1;
                //println!("entry 1 is {}", entry.1);
                game.remove(
                    game.iter()
                        .position(|x| x.1 == entry.1)
                        .expect("needle not found"),
                );
                //game.remove(index - (i + 2));
                break;
            }
        }
        // value was not present before
        if last_time_seen == 0 {
            game.push((0, index + 1));
            last_value = 0;
            //println!("last value was first time inserted {}", index);
            continue;
        }
        last_value = (index - last_time_seen) as i32;
        game.push((last_value, index + 1));
        //println!("last value was seen at time {} before", last_time_seen);
    }

    for entry in game {
        println!("{} {}", entry.0, entry.1);
    }

    println!("Solution is {}", last_value);
}
