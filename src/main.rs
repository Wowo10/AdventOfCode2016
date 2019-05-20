//https://www.samouczekprogramisty.pl/advent-of-code-2016-dzien-4/

extern crate regex;
use regex::Regex;
use std::fmt;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_data() -> Vec<String> {
    let mut data = Vec::new();

    let file = File::open("./res/rooms.txt").unwrap();
    for line in BufReader::new(file).lines() {
        data.push(line.unwrap());
    }

    data
}

struct Room {
    id: String,
    checksum: String,
}

impl Room {
    fn create(data: &String) -> Self {
        let re = Regex::new(r"\[(.*?)\]").unwrap();

        let checksum = re.captures(&data).unwrap().get(1).unwrap();

        let re = Regex::new(r"(^.+?)\[").unwrap();

        let id = re.captures(&data).unwrap().get(1).unwrap();

        Room {
            id: String::from(id.as_str()),
            checksum: String::from(checksum.as_str()),
        }
    }

    
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Room: {}[{}]", self.id, self.checksum)
    }
}

fn load_rooms(data: &Vec<String>) -> Vec<Room> {
    data.iter().map(|data_row| Room::create(data_row)).collect()
}

fn main() {
    let lines = load_rooms(&load_data());

    for line in lines {
        println!("{}", line);
    }


}
