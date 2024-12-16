use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

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

    pub fn is_valid_part1(&self, position: &Position, antena: &char) -> bool {
        if !self.in_bound(&position) || !self.is_antena(&position, antena) {
            return false
        }
        true
    }

    pub fn is_valid_part2(&self, position: &Position) -> bool {
        if !self.in_bound(&position) { 
            return false
        }
        true
    }

    fn in_bound(&self, position: &Position) -> bool {
        if position.row >= 0 && position.row < self.rows as i32 && position.col >= 0 && position.col < self.cols as i32 {
            return true
        }
        false
    }

    fn is_antena(&self, position: &Position, antena: &char) -> bool {
        let value = self.grid[position.row as usize][position.col as usize];
        if value == *antena {
            return false
        }
        true
    }

}

fn generate_anti_node(p1: &Position, p2: &Position) -> (Position, Position) {
    let height =  p2.row - p1.row;
    let length = p2.col - p1.col;

    let new_position1 = Position{ row: p1.row - height, col: p1.col - length };
    let new_position2 = Position{ row: p2.row + height, col: p2.col + length };

    return (new_position1, new_position2)
}

fn generate_anti_node_part2(p1: &Position, p2: &Position, max_rows: i32, max_cols: i32) -> Vec<Position> {
    let height =  p2.row - p1.row;
    let length = p2.col - p1.col;
    let mut positions: Vec<Position> = vec![p1.clone(), p2.clone()];

    let mut curr_row = p1.row - height;
    let mut curr_col= p1.col - length;
    while curr_row >= 0 && curr_col >= 0  {
        let new_position = Position{ row: curr_row, col: curr_col };
        positions.push(new_position);
        curr_row -= height;
        curr_col -= length;
    }

    let mut curr_row = p2.row + height;
    let mut curr_col= p2.col + length;
    while  curr_row < max_rows && curr_col < max_cols {
        let new_position = Position{ row: curr_row, col: curr_col };
        positions.push(new_position);
        curr_row += height;
        curr_col += length;
    }

    positions
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

fn find_same_frequency(city: &Grid) -> HashMap<char, Vec<Position>> {
    let mut freq_map: HashMap<char, Vec<Position>> = HashMap::new();
    for row in 0..city.rows {
        for col in 0..city.cols {
            let value = city.grid[row][col];
            if !value.is_alphanumeric() {
                continue
            }
            let position = Position{row: row as i32, col: col as i32};
            freq_map
                .entry(value.clone())
                .or_insert_with(Vec::new)
                .push(position);
        }
    }

    freq_map
}

fn part1(city: &Grid, antena_map: &HashMap<char, Vec<Position>>) -> usize {
    let mut seen: HashSet<Position> = HashSet::new();
    for (antena, positions) in antena_map {
        for i in 0..positions.len() {
            for j in i..positions.len() {
                let (new_position1, new_position2) = generate_anti_node(&positions[i], &positions[j]) ;
                if city.is_valid_part1(&new_position1, antena) && !seen.contains(&new_position1) {
                    seen.insert(new_position1);
                }
                if city.is_valid_part1(&new_position2, antena) && !seen.contains(&new_position2) {
                    seen.insert(new_position2);
                }
            }
        }

    }

    return seen.len()
}

fn part2(city: &Grid, antena_map: &HashMap<char, Vec<Position>>) -> usize {
    let mut seen: HashSet<Position> = HashSet::new();
    for (_, positions) in antena_map {
        for i in 0..positions.len() {
            for j in i+1..positions.len() {
                let new_positions = generate_anti_node_part2(&positions[i], &positions[j], city.rows as i32, city.cols as i32);
                for new_position in new_positions {
                    if city.is_valid_part2(&new_position) && !seen.contains(&new_position) {
                        seen.insert(new_position);
                    }
                }
            }
        }

    }

    return seen.len()
}


fn main() -> Result<()> {
    println!("Hello, world!");
    let file = File::open("data/data.txt")?;
    let reader: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    let city= Grid::new(reader)?;
    let freq_map = find_same_frequency(&city);

    let part1_start_time = Instant::now();
    let part1_solution = part1(&city, &freq_map);
    let part1_end_time = part1_start_time.elapsed();
    println!("Part 1: Solution: {}\t Time: {:?}", part1_solution, part1_end_time); // 361.083 microsenconds

    let part2_start_time = Instant::now();
    let part2_solution = part2(&city, &freq_map);
    let part2_end_time = part2_start_time.elapsed();
    println!("Part 2: Solution: {}\t Time: {:?}", part2_solution, part2_end_time); // 2.110 millienconds

    Ok(())
}
