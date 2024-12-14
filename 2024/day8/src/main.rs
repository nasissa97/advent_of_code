use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum GridError {
    #[error("Postion ({x}, {y}) is out of bounds")]
    OutOfBounds { x: i32, y: i32 },
    #[error("Invalid character '{0}' found in grid")]
    InvalidCharacter(char),
    #[error("Grid cannot be empty")]
    EmtpyGrid,
}

#[derive(Clone, Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<char>>
}

impl Grid {
    pub fn new(grid: Vec<Vec<char>>) -> Result<Grid, GridError> {
        if grid.len() == 0 {
            return Err(GridError::EmtpyGrid)

        }
        let rows = grid.len();
        let cols = grid[0].len();
        Ok(Grid {
            rows,
            cols,
            grid,
        })
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let file = File::open("data/sample.txt")?;
    let reader: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    let grid = Grid::new(reader).context("Could not create grid");
    println!("{:?}", grid);

    Ok(())
}
