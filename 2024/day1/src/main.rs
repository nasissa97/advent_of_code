use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;


fn part1() -> Result<u32, String> {
    let file = File::open("data/data1.txt").expect("Can't find file 'test.txt'");
    let buf_reader = io::BufReader::new(file);
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in buf_reader.lines() {
        match line {
            Ok(line) => {
                let values  = line.split_whitespace().collect::<Vec<&str>>();
                if values.len() == 2 {
                    let left_val = values[0].parse::<u32>().unwrap();
                    let right_val = values[1].parse::<u32>().unwrap();
                    left_heap.push(Reverse(left_val));
                    right_heap.push(Reverse(right_val));
                }

            },
            Err(err) => println!("Error: {:?}", err.kind()),
        }
    }

    let mut ans= 0;

    if left_heap.len() != right_heap.len() {
        return Err("Length aren't the same".to_string());
    }

    for _ in 0..left_heap.len() {
        let left_min = left_heap.pop().unwrap().0;
        let right_min = right_heap.pop().unwrap().0;
        let diff = right_min.abs_diff(left_min);
        ans += diff;
    }
    Ok(ans)

}

fn part2() -> Result<u32, String> {
    let file = File::open("data/data2.txt").expect("Failed to open file");
    let buf_reader = BufReader::new(file);
    let mut freq: HashMap<u32, u32> = HashMap::new();
    let mut left_side: Vec<u32> = Vec::new();
    let mut right_side: Vec<u32> = Vec::new();
    for line in buf_reader.lines() {
        match line {
            Ok(line) => {
                let values = line.split_whitespace().collect::<Vec<&str>>();
                if values.len() == 2 {
                    let left_value = values[0].parse::<u32>().unwrap();
                    let right_value = values[1].parse::<u32>().unwrap();
                    freq.insert(left_value, 0);
                    left_side.push(left_value);
                    right_side.push(right_value);
                }

            }
            Err(e) => {
                println!("ERROR: {:?}", e.kind());
                return Err(e.to_string());
            }
        }
    }

    let mut ans = 0;
    while right_side.len() > 0 {
        let right_value = right_side.pop().unwrap();
        if freq.contains_key(&right_value) {
            *freq.get_mut(&right_value).unwrap() += 1;
        }
    }

    for value in left_side {
        match freq.get_key_value(&value) {
            Some(pair) => {
                let curr = pair.0 * pair.1;
                ans += curr
            },
            None => ans += 0,
        }
    }

    Ok(ans)
}

fn main() {
    let ans1 = part1().expect("Failed to run part1");
    let ans2 = part2().expect("Failed to run part2");
    println!("Answer to Part1: {}\n Anser to Part2: {}", ans1, ans2);
}
