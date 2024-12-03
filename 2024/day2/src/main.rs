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

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in report.windows(2) {
        let diff = window[1] - window[0];
        if !(1..=3).contains(&diff) {
            increasing = false;
        }
        if !(-3..=-1).contains(&diff) {
            decreasing = false
        }
    }

    increasing || decreasing
}

fn check_with_remove(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }
    false
}

fn main() {
    let file = File::open("data/data1.txt").unwrap();
    let buffer = BufReader::new(file);
    let mut ans = 0;
    for line in buffer.lines() {
        let value = line.unwrap();
        let numbers: Vec<i32>  = value
            .split_whitespace()
            .map(|v: &str| v.parse::<i32>().unwrap())
            .collect();

        if is_safe(&numbers) || check_with_remove(&numbers) {
            ans += 1;
        }


    }
    println!("Answer: {}", ans);
}
