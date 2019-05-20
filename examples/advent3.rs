//https://www.samouczekprogramisty.pl/advent-of-code-2016-dzien-3/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_data() -> Vec<String> {
    let mut data = Vec::new();

    let file = File::open("./res/triangles.txt").unwrap();
    for line in BufReader::new(file).lines() {
        data.push(line.unwrap());
    }

    data
}

fn load_edges(lines: &Vec<String>) -> Vec<[u32; 3]> {
    let mut triangles = Vec::new();

    for line in lines {
        let edges: Vec<String> = line.rsplit(';').map(|s| s.to_string()).collect();

        triangles.push([
            edges[0].parse().unwrap(),
            edges[1].parse().unwrap(),
            edges[2].parse().unwrap(),
        ]);
    }

    triangles
}

fn check_triangularity(data: &[u32; 3]) -> bool {
    if data[0] + data[1] < data[2] {
        false
    } else if data[0] + data[2] < data[1] {
        false
    } else if data[2] + data[1] < data[0] {
        false
    } else {
        true
    }
}

fn main() {
    let triangles = load_edges(&load_data());

    let mut counter = 1;
    for triangle in triangles {
        println!(
            "{}. {:?} : {}",
            counter,
            triangle,
            if check_triangularity(&triangle) {
                "a Triangle!"
            } else {
                "Not a Triangle!"
            }
        );
        counter += 1;
    }
}
