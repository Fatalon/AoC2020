//use itertools::Itertools;
use array_tool::vec::*;
use std::fs::File;
use std::io::{BufRead, BufReader}; // 0.9.0
                                   // game input array of u8
                                   // array of boards - a board is 5x5 u8 array -> index into array marks a table

// to search for a hit:
// 1. create array of possible wins (rows and columns) 5xu8 array
// 2. create lookup map for win to board
// 3. run loop that intersects starting at 5 with poluting game array (0..5, 0..6,.. ) with all possible wins, until one has size 5
// 4. calculate win
#[derive(Clone)]
struct Board {
    table: Vec<Vec<u8>>,
    id: usize,
}

fn build_board(board: Vec<String>, index: usize) -> Board {
    let table = board
        .into_iter()
        .map(|r| {
            r.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    return Board {
        id: index,
        table: table,
    };
}

fn read_file() -> Vec<String> {
    let f = BufReader::new(File::open("src/input.txt").unwrap());
    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    return lines;
}

fn task1() {}

fn main() {
    let lines: Vec<String> = read_file();
    let game_input: Vec<u8> = lines[0]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u8>>();
    let mut boards: Vec<Board> = vec![];
    let table_count = (lines.len() - 2) / 6;
    for index in 0..table_count {
        let start_index = 2 + (index * 6);
        boards.push(build_board(
            lines[start_index..start_index + 5].to_vec(),
            index,
        ));
    }
    // println!("Created game input {}, {}", game_input.len(), game_input[5]);
    // for board in &boards {
    //     println!("Board with id {}", board.id);
    //     for x in &board.table {
    //         for y in x {
    //             print!("{} ", y);
    //         }
    //         println!();
    //     }
    // }
    println!("Created boards {}", &boards.len());
    let mut possible_wins: Vec<Vec<u8>> = vec![];
    for board in &boards {
        for x in board.table.clone() {
            possible_wins.push(x);
        }
        for y in 0..board.table.len() {
            let mut v: Vec<u8> = vec![];
            for x in 0..board.table.len() {
                //println!("board {}", board.table[x][y]);
                v.push(board.table[x][y]);
            }
            possible_wins.push(v);
        }
    }
    println!("found possible {} solutions", possible_wins.len());

    //now draw game until solution found
    let mut solution: usize = 50000;
    let mut current_index: usize = 5;
    let mut final_number: u8;

    let mut solved_boards: Vec<usize> = vec![];
    for i in 0..20 {
        println!("##########ROUND {} #################", i);
        for (pos, possible_win) in possible_wins.iter().enumerate() {
            let intersect_array = possible_win.intersect(game_input[0..current_index].to_vec());
            // print!("[ ");
            // for e in &intersect_array {
            //     print!("{}, ", e);
            // }
            // print!(" ]");
            let size = intersect_array.len();
            // println!("\t\tsize was {}", size);
            if size == 5 {
                solution = pos / 10;
                println!(
                    "found solution at index {}, belonging to board {}",
                    pos, solution
                );
                //calculate result
                final_number = game_input[current_index - 1];
                println!("was number {}", final_number);
                let mut total_unmarked: u32 = 0;
                for x in boards[solution]
                    .clone()
                    .table
                    .into_iter()
                    .flatten()
                    .collect::<Vec<u8>>()
                {
                    if !game_input[0..current_index].to_vec().contains(&x) {
                        total_unmarked += x as u32;
                    }
                }
                println!(
                    "total number was {}, result is {}",
                    total_unmarked,
                    total_unmarked * final_number as u32
                );
            }
        }
        current_index += 1;
        if solution != 50000 {
            break;
        }
    }
}
