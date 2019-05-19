//https://www.samouczekprogramisty.pl/advent-of-code-2016-dzien-1/

use std::fmt;
use std::fs;
fn load_path() -> Vec<String> {
    let contents = fs::read_to_string("./res/path.txt").expect("No File found.");

    contents.rsplit(',').map(|s| s.to_string()).collect()
}

enum Direction {
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "{}", 'L'),
            Direction::Right => write!(f, "{}", 'R'),
        }
    }
}

struct Move {
    pub direction: Direction,
    pub steps: i32,
}

impl Move {
    pub fn create(data: String) -> Self {
        let dir = match &data[..1] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => Direction::Left,
        };

        let steps = &data[1..];

        Move {
            direction: dir,
            steps: steps.parse().expect("Oh No you don`t."),
        }
    }
}

enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn(&mut self, direction: &Direction) -> Heading {
        match direction {
            Direction::Left => self.turn_left(),
            Direction::Right => self.turn_right(),
        }
    }

    fn turn_left(&mut self) -> Heading {
        match self {
            Heading::North => Heading::West,
            Heading::West => Heading::South,
            Heading::South => Heading::East,
            Heading::East => Heading::North,
        }
    }

    fn turn_right(&mut self) -> Heading {
        match self {
            Heading::North => Heading::East,
            Heading::West => Heading::North,
            Heading::South => Heading::West,
            Heading::East => Heading::South,
        }
    }
}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Heading::North => write!(f, "{}", "North"),
            Heading::West => write!(f, "{}", "West"),
            Heading::South => write!(f, "{}", "South"),
            Heading::East => write!(f, "{}", "East"),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
    heading: Heading,
}

impl Position {
    fn create() -> Self {
        Position {
            x: 0,
            y: 0,
            heading: Heading::North,
        }
    }

    fn change(&mut self, mov: &Move) {
        self.heading = self.heading.turn(&mov.direction);

        match self.heading {
            Heading::North => {
                self.x += mov.steps;
            }
            Heading::West => {
                self.y -= mov.steps;
            }
            Heading::South => {
                self.x -= mov.steps;
            }
            Heading::East => {
                self.y += mov.steps;
            }
        }
    }

    fn distance(&self) -> f64 {
        let distance: f64 = (self.x * self.x) as f64 + (self.y * self.y) as f64;

        distance.sqrt()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "position: ({}, {}), heading: {}",
            self.x, self.y, self.heading
        )
    }
}

fn main() {
    let path = load_path();

    println!("{:?}", path);

    let mut moves = Vec::new();

    for move_str in path {
        moves.push(Move::create(move_str));
    }

    for mov in &moves {
        println!("dir:{}, steps:{}", mov.direction, mov.steps);
    }

    let mut position = Position::create();

    for mov in &moves {
        position.change(mov);
        println!("{}, distance: {}", position, position.distance());
    }

    
}
