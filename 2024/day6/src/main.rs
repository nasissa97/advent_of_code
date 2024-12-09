use std::ops::Add;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
enum Day6Error {
    #[error("IO is tripping")]
    Io(#[from] io::Error),
    #[error("Custom error: {0}")]
    Custom(String),
}

#[derive(Hash, Eq, Clone, PartialEq, Debug)]
struct Position {
    pub row: i64,
    pub col: i64,
}

const UP_POSITION:Position = Position{row: -1, col: 0};
const DOWN_POSITION:Position = Position{row: 1, col: 0};
const LEFT_POSITION:Position = Position{row: 0, col: -1};
const RIGHT_POSITION:Position = Position{row: 0, col: 1};

impl Position {
    pub fn new(row: i64, col: i64) -> Position {
        Position {
            row,
            col
        }
    }

    pub fn get_position(&self) -> &Position {
        todo!()
    }
}

impl <'a, 'b>Add<&'b Position> for &'a Position {
    type Output = Position;
    fn add(self, rhs: &Position) -> Position {
        Position {
            row: self.row + rhs.row,
            col: self.col + rhs.col
        }
    }
}

// Not the best idea to expose all the field but then its Advent 🤷
#[derive(Debug, Clone)]
struct Guard {
    pub position: Position,
    pub current_move: Position,
    pub seen: HashSet<Position>,
    pub in_loop: bool,
}

impl Guard {
    pub fn new(position: Position) -> Guard {
        let mut seen = HashSet::new();
        seen.insert(position.clone());
        let current_move = Position::new(-1, 0);
        let in_loop = false;
        Guard {
            position,
            current_move,
            seen,
            in_loop
        }
    }

    pub fn next_move(&self) -> Position {
        &self.position + &self.current_move
    }

    pub fn update_position(&mut self, new_position: Position) {
        // if self.seen.contains(&new_position) {
            // println!("Seen Position: {:?}", &new_position);
        // }
        self.seen.insert(new_position.clone());
        self.position = new_position;
    }

    pub fn update_move(&mut self) {
        if self.in_loop {
            self.in_loop = false;
        }
        match self.current_move {
            UP_POSITION =>  self.current_move = RIGHT_POSITION,
            RIGHT_POSITION => self.current_move = DOWN_POSITION,
            DOWN_POSITION => self.current_move = LEFT_POSITION,
            LEFT_POSITION => self.current_move = UP_POSITION,
            _ => println!("Current Move has invalid state: {:?}", self.current_move), 
        }
        // println!("MOVED");
    }


    pub fn cause_loop(&self) -> bool {
        todo!();
    }

    pub fn look_over(&mut self) -> bool {
        if self.in_loop {
            return false
        }
        let direction = match self.current_move {
            UP_POSITION =>  Some(RIGHT_POSITION),
            RIGHT_POSITION => Some(DOWN_POSITION),
            DOWN_POSITION => Some(LEFT_POSITION),
            LEFT_POSITION => Some(UP_POSITION),
            _ => None
        };

        if direction.is_none() {
            return false
        }

        let direction = direction.unwrap();

        self.in_loop = true;

        true
    }


}


struct Classroom {
    pub grid: Vec<Vec<String>>,
    pub rows: i64,
    pub cols: i64,
}

impl Classroom {
    pub fn new(grid: Vec<Vec<String>>, rows: i64, cols: i64) -> Classroom {
        Classroom {
            grid,
            rows,
            cols,
        }
    }


    pub fn is_valid(&self, position: &Position) -> bool {
        if position.row < 0 || position.row >= self.rows {
            return false
        }
        if position.col < 0 || position.col >= self.cols {
            return false
        }

        let row = position.row as usize;
        let col = position.col as usize;
        let value = self.grid.get(row).unwrap().get(col).unwrap().as_str();
        if value == "#" {
            return false
        }
        true
    }

    pub fn has_exit(&self, position: &Position) -> bool {
        if position.row < 0 || position.row >= self.rows-1 {
            return true 
        }
        if position.col < 0 || position.col >= self.cols-1 {
            return true 
        }
        false
    }

    pub fn detect_loop(&self, guard: &mut Guard) -> bool {
        let mut on_path = false;
        while self.is_valid(&guard.position) {
            let next_move = &guard.position + &guard.current_move;
            if !guard.seen.contains(&next_move) {
                break
            }
            guard.update_position(next_move);
            if guard.seen.contains(&guard.position) {
                on_path = true;
            }
        }
        let position = &guard.position + &guard.current_move;
        let row = position.row;
        let col = position.col;
        let value  = self.grid.get(row as usize);
        if value.is_none() {
            return false
        }
        let value = value.unwrap().get(col as usize).clone();
        if value.is_none() {
            return false
        }

        let value = value.unwrap().as_str();
        if on_path && value == "#" {
            return true
        }

        false
    }


}

fn part1(grid: Vec<Vec<String>>, rows: i64, cols: i64, position: Position) -> u64 {
    let mut guard = Guard::new(position);
    let classroom = Classroom::new(grid, rows as i64, cols as i64);
    let mut count = 0;
    while !classroom.has_exit(&guard.position) {
        let next_move = guard.next_move();
        let mut guard_clone = guard.clone();
        guard_clone.update_move();
        if classroom.detect_loop(&mut guard_clone) {
            println!("Potential Loop: {:?}", &guard_clone.position);
        }
        if classroom.is_valid(&next_move) {
            guard.update_position(next_move);
        } else {
            guard.update_move();
        }

        count += 1;
        // For testing purpose
        if count > 10000 {
            println!("You might of got into a infinite loop :/");
            break;
        }
    }

    guard.seen.len() as u64
}

fn main() -> Result<(), Day6Error> {
    println!("Hello, world!");
    let file = File::open("data/sample1.txt")?;
    let reader = BufReader::new(file);
    let mut classroom: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<String> = line
            .split("")
            .filter(|&s| !s.is_empty())
            .map(|ch| ch.to_string())
            .collect();

        classroom.push(row);
    }

    let rows = classroom.len();
    if rows == 0 {
        return Err(Day6Error::Custom("No Rows You Must of Read File Wrong".into()))
    }
    let cols = classroom.get(0).unwrap().len();
    let mut starting_position: Option<Position> = None;

    for (row, row_values) in classroom.iter().enumerate() {
        for (col, val) in row_values.iter().enumerate() {
            if val == "^" {
                println!("Row: {:?} Col {}", row, col);
                starting_position = Some(Position::new(row as i64, col as i64));
            }

        }
    }
    match starting_position {
        Some(position) => {
            let anwser = part1(classroom, rows as i64, cols as i64, position);
            println!("Answer {}", anwser)
        }
        None => println!("Couldn't find the guard!"),
    }
    Ok(())
}
