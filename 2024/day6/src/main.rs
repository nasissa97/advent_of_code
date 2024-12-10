use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;
use anyhow::{Result, Context};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }

    fn move_forward(self, pos: &Position) -> Position {
        match self {
            Direction::Up => Position { x: pos.x, y: pos.y - 1},
            Direction::Down => Position { x: pos.x, y: pos.y + 1},
            Direction::Left => Position { x: pos.x - 1, y: pos.y },
            Direction::Right => Position { x: pos.x + 1, y: pos.y },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    direction: Direction,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.hash(state)
    }
}

#[derive(Debug, Error)]
enum GridError {
    #[error("Position ({x}, {y}) is out of bounds")]
    OutOfBounds { x: i32, y: i32 },
    #[error("Invalid character '{0}' found in grid")]
    InvalidCharacter(char),
}

#[derive(Clone, Debug)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<char>>
}

impl Grid {
    fn is_open(&self, pos: &Position) -> Result<bool> {
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= self.width || pos.y as usize >= self.height {
            return Err(GridError::OutOfBounds { x: pos.x, y: pos.y }.into())
        }
        let tile = self.tiles[pos.y as usize][pos.x as usize];
        if tile != '.' && tile != '#' {
            return Err(GridError::InvalidCharacter(tile).into());
        }
        Ok(tile == '.')
    }
}

fn simulate_guard_path(grid: &Grid, start: Position, mut direction: Direction) -> Result<HashSet<Position>> {
    let mut visited = HashSet::new();
    let mut current_position = start.clone();

    visited.insert(current_position.clone());

    loop {
        // Determine the next position based on the current direction
        let next_position = direction.move_forward(&current_position);

        // Exit the loop if the guard moves out of bounds
        if next_position.x < 0 || next_position.y < 0
            || next_position.x as usize >= grid.width
            || next_position.y as usize >= grid.height
        {
            break;
        }

        // Check if the next position is open
        if grid.is_open(&next_position)? {
            // Move to the next position and mark it as visited
            current_position = next_position;
            visited.insert(current_position.clone());
        } else {
            // Turn right if the next position is blocked
            direction = direction.turn_right();
        }
    }

    Ok(visited)
}

fn simulate_guard_path_with_loop_detection(
    grid: &Grid,
    start: Position,
    mut direction: Direction,
    obstruction: Option<&Position>,
) -> Result<bool> {
    let mut visited_states = HashSet::new();
    let mut current_position = start.clone();
    loop {
        // Add the current state to visited set
        let state = State{position: current_position.clone(), direction: direction.clone()};
        if visited_states.contains(&state) {
            return Ok(true)
        }
        visited_states.insert(state);

        let next_position = direction.move_forward(&current_position);

        if next_position.x < 0 || next_position.y < 0 
            || next_position.x as usize >= grid.width
            || next_position.y as usize >= grid.height
        {
            return Ok(false)
        }

        if let Some(obs) = obstruction {
            if next_position == obs.clone() {
                direction = direction.turn_right();
                continue;
            }
        }

        if grid.is_open(&next_position)? {
            current_position = next_position;
        } else {
            direction = direction.turn_right();
        }
    }
}

/// Determines valid obstruction positions that would cause the guard to loop.
fn find_obstruction_positions(grid: &Grid, guard_start: Position, direction: Direction) -> Result<Vec<Position>> {
    let mut valid_positions = Vec::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let candidate = Position { x: x as i32, y: y as i32 };

            // Skip starting position
            if candidate == guard_start || !grid.is_open(&candidate)? {
                continue
            }

            // Simulate and check for loop
            let mut obstructed_grid = grid.clone();
            obstructed_grid.tiles[candidate.y as usize][candidate.x as usize] = '#';

            let detected_loop= simulate_guard_path_with_loop_detection(
                &obstructed_grid, 
                guard_start.clone(), 
                direction.clone(),
                Some(&candidate),
            )?;
            if detected_loop {
                valid_positions.push(candidate);
            }
        }
    }
    Ok(valid_positions)
}


fn main() -> Result<()> {
    println!("Hello, world!");
    let file = File::open("data/data.txt")?;
    let reader = BufReader::new(file);
    let mut tiles: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line
            .chars()
            .collect();

        tiles.push(row);
    }


    let mut guard_position: Option<Position> = None;
    for (row, row_values) in tiles.iter().enumerate() {
        for (col, val) in row_values.iter().enumerate() {
            if *val == '^' {
                println!("Row: {:?} Col {}", row, col);
                guard_position = Some(Position{x: col as i32, y: row as i32});
                break
            }
        }
        if guard_position.is_some() {
           break;
        }
    }

    

    let guard_start = match guard_position {
        Some(pos) => pos,
        None => return Ok(()),
    };

    let guard_direction = Direction::Up;

    tiles[guard_start.y as usize][guard_start.x as usize] = '.';

    let grid = Grid {
        width: tiles[0].len(),
        height: tiles.len(),
        tiles,
    };


    let visited_positions = simulate_guard_path(&grid, guard_start.clone(), guard_direction.clone())
        .context("Failed to simulate guard path for Part1")?;
    println!("Part 1: Distinct positions visited: {}", visited_positions.len());

    // Part 2: Find valid obstruction positions
    let now = Instant::now();
    let valid_positions = find_obstruction_positions(&grid, guard_start.clone(), guard_direction.clone())
        .context("Failed to find valid obstruction positions for Part 2")?;
    // println!("Part 2: Valid obstructions positions: {:?}", valid_positions);
    let elapsed = now.elapsed();
    // Synchronous => 96.74s
    // Asynchronus => 
    println!("Elapsed: {:.2?}", elapsed);
    println!("Part 2: Number of valid positions: {}", valid_positions.len());
    Ok(())
}
