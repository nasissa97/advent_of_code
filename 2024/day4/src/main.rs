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

fn bfs(grid: &Vec<Vec<String>>, row: usize, col: usize) -> u8 {

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut path: Vec<String> = vec!['S'.into(), 'A'.into(), 'M'.into(), 'X'.into()];
    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();
    let mut total = 0;

    queue.push_back((row, col));
    // let mut levels = 1;
    let mut layer = path.pop().unwrap();
    if grid.get(row).unwrap().get(col).unwrap().to_string() != layer.to_string() {
        return 0
    }

    while queue.len() > 0 {
        println!("{:?}", queue);
        let n = queue.len();
        // if levels >= path.len() {
            // break;
        // }
        if path.len() == 0 {
            return total
        }
        layer = path.pop().unwrap();

        for _ in 0..n {
            let (r, c) = queue.pop_front().unwrap();
            let node_val = grid.get(r).unwrap().get(c).unwrap();
            println!("Node: {:?}, Layer {}", (r, c), layer);
            for neighbor in get_neighbors(r, c, rows, cols) {
                let (next_row, next_col) = neighbor;
                let next_val = grid.get(next_row).unwrap().get(next_col).unwrap();
                if layer.as_str() == "S" && next_val.as_str() == "S" {
                    total += 1;
                    break;
                }
                println!("{} == {}", layer, *next_val);
                if *next_val == layer && layer.as_str() != "S" {
                    queue.push_back((next_row, next_col));
                }
            }
        }
        // levels += 1;
    }
    total
}

fn is_xmas(grid: &Vec<Vec<String>>, checks: Vec<(i32, i32)>, row: usize, col: usize) -> bool {
    let path = vec!["M", "A", "S"];
    let mut is_xmas = true;
    for (idx, (y, x)) in checks.iter().enumerate() {
        let next_row = (row as i32 + y) as usize;
        let next_col = (col as i32 + x) as usize;
        println!("Next Row {}\nNext Col {}", next_row, next_col);
        let value = grid.get(next_row).unwrap().get(next_col).unwrap().to_string();
        let want =  path.get(idx).unwrap().to_string();
        if value != want {
            is_xmas = false;
            break;
        }
    }

    is_xmas
}

fn check_neighbors(grid: &Vec<Vec<String>>, row: i32, col: i32) -> u8 {
    let mut ans = 0;
    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();
    if row + 3 < rows as i32 {
        let checks = vec![(1,0), (2,0), (3,0)];
        if is_xmas(grid, checks, row as usize, col as usize) {
            ans += 1;
        }
    }
    if row - 3 >= 0 {
        let checks = vec![(-1,0), (-2,0), (-3,0)];
        if is_xmas(grid, checks, row as usize, col as usize) {
            ans += 1;
        }

    }
    if col + 3 < cols as i32 {
        let checks = vec![(0,1), (0,2), (0,3)];
        if is_xmas(grid, checks, row as usize, col as usize) {
            ans += 1;
        }

    }
    if col - 3 >= 0 { 
        let checks = vec![(0,-1), (0,-2), (0,-3)];
        if is_xmas(grid, checks, row as usize, col as usize) {
            ans += 1;
        }

    }

    if col + 3 < cols as i32  && row - 3 < rows as i32 {
        let checks = vec![(-1,1), (-2,2), (-3,3)];
        if is_xmas(grid, checks, row as usize, col as usize) {
            ans += 1;
        }

    }

    if col + 3 < cols as i32 && row + 3 < rows as i32 {
        let checks = vec![(1,1), (2,2), (3,3)];
        if is_xmas(grid, checks, row as usize, col as usize) {
            ans += 1;
        }

    }

    if col - 3 >= 0 && row + 3 < rows as i32 {
        let checks = vec![(1,-1), (2,-2), (3,-3)];
        if is_xmas(grid, checks, row as usize, col as usize) {
            ans += 1;
        }

    }

    if col - 3 < 0 && row - 3 >= 0 {
        let checks = vec![(-1,-1), (-2,-2), (-3,-3)];
        if is_xmas(grid, checks, row as usize, col as usize) {
            ans += 1;
        }

    }

    ans

}

fn main() -> anyhow::Result<()> {
    let file = File::open("data/sample1.txt")?;
    let reader = BufReader::new(file);
    let mut grid: Vec<Vec<String>> = vec![];

    // Build grid
    for line in reader.lines() {
        // let row = line?.chars().collect();
        let values = line?;
        let mut row: Vec<String> = vec![];
        for val in values.chars() {
            row.push(val.to_string());
        }
        grid.push(row);
    }


    let rows = grid.len();
    let cols = grid.get(0).unwrap().len();
    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols  {
            let val = grid.get(row).unwrap().get(col).unwrap();
            if val.to_string() == "X".to_string()  {
                println!("Row: {} Col: {}", row, col);
                // let total = bfs(&grid, row, col);
                let total = check_neighbors(&grid, row as i32, col as i32);
                ans += total;
            }
        }
        println!("");
        break;
    }

    println!("Answer: {}", ans);
    Ok(())
}
