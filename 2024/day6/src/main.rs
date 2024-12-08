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

fn next_move(current: (i8, i8)) -> (i8, i8) {
    match current {
        (-1,0) =>  (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => current
    }
}

fn traverse(grid: &mut Vec<Vec<String>>, row: usize, col: usize, rows: u8, cols: u8) -> u64 {
    todo!()
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
    // let cols = classroom.get(0).unwrap().len();

    for row in classroom.iter() {
        for (col, val) in row.iter().enumerate() {
            if val == "^" {
                print!("Row: {:?} Col {}", row, col);
                return Ok(())
            }

        }
    }

    println!("{:?}", classroom);
    Ok(())
}
