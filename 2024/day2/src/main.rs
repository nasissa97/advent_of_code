use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_sequence_valid<F>(numbers: &[u32], compare: F) -> bool 
where
    F: Fn(u32, u32) -> bool,
{
    for windows in numbers.windows(2) {
        let (a, b) = (windows[0], windows[1]);
        if !compare(a, b) || b.abs_diff(a) > 3 {
            return false
        }
    }

    true
}

fn is_decreasing(a: u32, b: u32) -> bool {
    a > b
}

fn is_increasing(a: u32, b: u32) -> bool {
    a < b 
}

fn determine_order(numbers: &[u32]) -> Option<&'static str> {
    if numbers.len() < 2 {
        return None; // Not enought elements to determine order.
    }

    let first = numbers[0];
    let second = numbers[1];

    if second > first {
        Some("increasing")
    } else if second < first {
        Some("decreasing")
    } else {
        None // Undefined Order
    }
}


fn main() {
    let file = File::open("data/data1.txt").unwrap();
    let buffer = BufReader::new(file);
    println!("Hello, world!");
    let mut ans = 0;
    for line in buffer.lines() {
        let value = line.unwrap();
        let numbers: Vec<u32> = value
            .split_whitespace()
            .map(|v: &str| v.parse::<u32>().unwrap())
            .collect();


        match determine_order(&numbers) {
            Some("increasing") => {
                if is_sequence_valid(&numbers, is_increasing) {
                    ans += 1;
                }
            }
            Some("decreasing") => {
                if is_sequence_valid(&numbers, is_decreasing) {
                    ans += 1
                }
            }
            _ => println!("Undefined Order")
        }
    }
    println!("Answer: {}", ans);
}
