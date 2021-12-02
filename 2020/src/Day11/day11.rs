use std::fs;

fn predictRound(waiting_area: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_waiting_area: Vec<Vec<char>> =
        vec![vec!['.'; waiting_area[0].len()]; waiting_area.len()];

    let mut longest_distance = waiting_area[0].len();
    if waiting_area.len() > waiting_area[0].len() {
        longest_distance = waiting_area.len();
    }

    let directions = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for x in 0..waiting_area.len() {
        let line = waiting_area[x].clone();
        for y in 0..line.len() {
            let space: char = waiting_area[x][y];
            //If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
            if space == 'L' {
                let mut counter = 0;

                let signed_x = x as isize;
                let signed_y = y as isize;

                for direction in directions.clone() {
                    for offset in 1..longest_distance {
                        let neigh_x = signed_x + direction.0 * offset as isize;
                        let neigh_y = signed_y + direction.1 * offset as isize;
                        if neigh_x < 0
                            || neigh_x as usize >= waiting_area.len()
                            || neigh_y < 0
                            || neigh_y as usize >= waiting_area[0].len()
                        {
                            break;
                        }
                        if waiting_area[neigh_x as usize][neigh_y as usize] == '#' {
                            counter += 1;
                            break;
                        } else if waiting_area[neigh_x as usize][neigh_y as usize] == 'L' {
                            break;
                        }
                    }
                }

                if counter == 0 {
                    new_waiting_area[x][y] = '#';
                } else {
                    new_waiting_area[x][y] = 'L';
                }
            } else if space == '#' {
                let mut counter = 0;
                let signed_x = x as isize;
                let signed_y = y as isize;

                for direction in directions.clone() {
                    for offset in 1..longest_distance {
                        let neigh_x = signed_x + direction.0 * offset as isize;
                        let neigh_y = signed_y + direction.1 * offset as isize;
                        if neigh_x < 0
                            || neigh_x as usize >= waiting_area.len()
                            || neigh_y < 0
                            || neigh_y as usize >= waiting_area[0].len()
                        {
                            break;
                        }
                        if waiting_area[neigh_x as usize][neigh_y as usize] == '#' {
                            counter += 1;
                            break;
                        } else if waiting_area[neigh_x as usize][neigh_y as usize] == 'L' {
                            break;
                        }
                    }
                }
                if counter >= 5 {
                    new_waiting_area[x][y] = 'L';
                } else {
                    new_waiting_area[x][y] = '#';
                }
            }
        }
    }
    return new_waiting_area;
}

fn check_if_areas_are_equal(
    waiting_area: Vec<Vec<char>>,
    new_waiting_area: Vec<Vec<char>>,
) -> bool {
    for x in 0..waiting_area.len() {
        for y in 0..waiting_area[0].len() {
            if waiting_area[x][y] != new_waiting_area[x][y] {
                println!(
                    "{}, {} was not equal with values {},{}",
                    x, y, waiting_area[x][y], new_waiting_area[x][y]
                );
                return false;
            }
        }
    }
    return true;
}

fn main() {
    // --snip
    let filename = "src/Day11/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut waiting_area: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let char_vec: Vec<char> = line.chars().collect();
        waiting_area.push(char_vec.clone());
    }

    let mut iterations = 0;
    loop {
        iterations += 1;
        let new_waiting_area: Vec<Vec<char>> = predictRound(waiting_area.clone());
        for x in new_waiting_area.clone() {
            for y in x {
                print!("{}", y);
            }
            println!();
        }
        println!();
        if check_if_areas_are_equal(waiting_area.clone(), new_waiting_area.clone()) {
            break;
        }
        waiting_area = new_waiting_area.clone();
    }

    println!("took {} iterations", iterations);

    let mut occupied_seats = 0;
    for x in waiting_area {
        for y in x {
            //print!("{}", y);
            if y == '#' {
                occupied_seats += 1;
            }
        }
        //println!();
    }
    println!("there are {} occupied seats", occupied_seats);
}
