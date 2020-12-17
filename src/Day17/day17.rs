use itertools::Itertools;
use std::fs;

const MAX_SIZE: usize = 30;
fn predictRound(game: Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut new_game_state: Vec<Vec<Vec<Vec<char>>>> =
        vec![vec![vec![vec!['.'; MAX_SIZE]; MAX_SIZE]; MAX_SIZE]; MAX_SIZE];

    let possible_neighbour_values: Vec<isize> = vec![-1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0, 1];
    let mut directions = Vec::new();
    for perm in possible_neighbour_values.iter().permutations(4).unique() {
        if perm[0] == perm[1] && perm[1] == perm[2] && perm[2] == perm[3] && perm[2] == &0 {
            continue;
        }
        directions.push(perm);
    }
    println!("direction has {} entries", directions.len());
    for w in 0..game.len() - 1 {
        let dimension = &game[w];
        for z in 0..dimension.len() - 1 {
            let level = dimension[z].clone();
            for x in 0..level.len() - 1 {
                let row = &level[x];
                for y in 0..row.len() - 1 {
                    let space: char = game[w][z][x][y];
                    //If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
                    if space == '#' {
                        let mut counter = 0;

                        for direction in directions.clone() {
                            if w as isize + direction[0] >= 0
                                && z as isize + direction[1] >= 0
                                && x as isize + direction[2] >= 0
                                && y as isize + direction[3] >= 0
                            {
                                let neighbour = game[(w as isize + direction[0]) as usize]
                                    [(z as isize + direction[1]) as usize]
                                    [(x as isize + direction[2]) as usize]
                                    [(y as isize + direction[3]) as usize];
                                if neighbour == '#' {
                                    counter += 1;
                                }
                                if counter > 3 {
                                    break;
                                }
                            }
                        }

                        if counter == 2 || counter == 3 {
                            new_game_state[w][z][x][y] = '#';
                        } else {
                            new_game_state[w][z][x][y] = '.';
                        }
                    //game cell is inactive
                    } else {
                        let mut counter = 0;

                        for direction in directions.clone() {
                            if w as isize + direction[0] >= 0
                                && z as isize + direction[1] >= 0
                                && x as isize + direction[2] >= 0
                                && y as isize + direction[3] >= 0
                            {
                                let neighbour = game[(w as isize + direction[0]) as usize]
                                    [(z as isize + direction[1]) as usize]
                                    [(x as isize + direction[2]) as usize]
                                    [(y as isize + direction[3]) as usize];
                                if neighbour == '#' {
                                    counter += 1;
                                }
                                if counter > 3 {
                                    break;
                                }
                            }
                        }
                        if counter == 3 {
                            new_game_state[w][z][x][y] = '#';
                        } else {
                            new_game_state[w][z][x][y] = '.';
                        }
                    }
                }
            }
        }
    }
    return new_game_state;
}

fn main() {
    // --snip--
    let filename = "src/Day17/input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.lines().collect();

    //z,x,y
    let mut game: Vec<Vec<Vec<Vec<_>>>> =
        vec![vec![vec![vec!['.'; MAX_SIZE]; MAX_SIZE]; MAX_SIZE]; MAX_SIZE];

    //need to ensure -1,0,1 instead of 0,1,2
    let offset = MAX_SIZE / 2;

    for (x, line) in lines.iter().enumerate() {
        for (y, cube) in line.chars().enumerate() {
            game[offset][offset][x + offset][y + offset] = cube;
        }
    }

    for round in 0..6 {
        println!("Round: {}", round);

        //et mut index = -(offset as isize);
        // for level in game.clone() {
        //     println!("level {}", index);
        //     for row in level {
        //         for c in row {
        //             print!("{}", c);
        //         }
        //         println!();
        //     }
        //     index += 1;
        // }
        game = predictRound(game.clone());
    }

    let mut counted_active = 0;
    for dimension in game.clone() {
        for level in dimension {
            for row in level {
                for c in row {
                    if c == '#' {
                        counted_active += 1;
                    }
                }
            }
        }
    }
    println!("counted active was {}", counted_active);
}
