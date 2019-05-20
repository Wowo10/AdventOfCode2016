//https://www.samouczekprogramisty.pl/advent-of-code-2016-dzien-2/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_codes() -> Vec<String> {
    let mut codes = Vec::new();

    let file = File::open("./res/code.txt").unwrap();
    for line in BufReader::new(file).lines() {
        codes.push(line.unwrap());
    }

    codes
}

fn main() {
    let code_table = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let codes = load_codes();

    let mut position_x = 1;
    let mut position_y = 1;

    for code in &codes {
        for c in code.chars() {
            match c {
                'U' => {
                    if position_y != 0 {
                        position_y -= 1;
                    }
                }
                'L' => {
                    if position_x != 0 {
                        position_x -= 1;
                    }
                }
                'D' => {
                    if position_y != 2 {
                        position_y += 1;
                    }
                }
                'R' => {
                    if position_x != 2 {
                        position_x += 1;
                    }
                }
                _ => {
                    //do nothing
                }
            }
        }
        print!("{}", code_table[position_y][position_x]);
    }
    println!("");
}
