use std::fs::File;
use std::io::{self, BufRead, BufReader};
use thiserror::Error;
use anyhow::Error as AnyhowError;
use std::collections::VecDeque;


#[derive(Error, Debug)]
enum Day4Error {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Other error: {0}")]
    Anyhow(#[from] AnyhowError),
    #[error("Custom error: {0}")]
    Custom(String),
}

fn get_neighbors(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let delta_row = vec![-1, -1, 0, 1,  1, 1, 0, -1];
    let delta_col  = vec![0, 1, 1, 1, 0, -1, -1, -1];
    let mut neighbors:Vec<(usize, usize)> = vec![];
    for i in 0..delta_col.len() {
        let next_row = row as i32 + delta_row.get(i).unwrap();
        let next_col = col as i32 + delta_col.get(i).unwrap();
        if next_row >= 0 && next_row < rows as i32 && next_col >= 0 && next_col < cols as i32 {
            neighbors.push((next_row as usize, next_col as usize));
        }
    }
    neighbors
}

fn bfs(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u8 {

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let path: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();
    queue.push_back((row, col));
    let mut levels = 1;
    let mut total = 0;
    while queue.len() > 0 {
        let n = queue.len();
        if levels >= path.len() {
            break;
        }
        for _ in 0..n {
            let (r, c) = queue.pop_front().unwrap();
            if grid.get(r).unwrap().get(c).unwrap().to_string() == "S".to_string() {
                total += 1;
            }
            for neighbor in get_neighbors(row, col, rows, cols) {
                let (next_row, next_col) = neighbor;
                let next_val = grid.get(next_row).unwrap().get(next_col).unwrap();
                println!("LEVELS: {}", levels);
                println!("VAL: {}", next_val);
                if next_val == path.get(levels).unwrap() {
                    queue.push_back((next_row, next_col));
                }
            }
        }
        levels += 1;
    }
    total
}

fn main() -> anyhow::Result<()> {
    let file = File::open("data/sample1.txt")?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<char>> = vec![];

    // Build grid
    for line in reader.lines() {
        let row = line?.chars().collect();
        grid.push(row);
    }


    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();
    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            let val = grid.get(row).unwrap().get(col).unwrap();
            if val.to_string() == "X".to_string()  {
                println!("Row: {} Col: {}", row, col);
                let total = bfs(&grid, row, col);
                ans += total;
            }
        }
        println!("");
        break;
    }

    println!("Answer: {}", ans);
    Ok(())
}
