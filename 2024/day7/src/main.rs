use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::time::Instant;

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum CalibrationError {
    #[error("string is in correct format to create caliberation")]
    InvalidString,
}


#[derive(Debug, Clone)]
struct Calibration {
    solution: u64,
    equation: Vec<u64>

}

impl Calibration {
    pub fn new(value: String) -> Result<Self, CalibrationError> {
        if !value.contains(":") {
            return Err(CalibrationError::InvalidString)
        }

        let nums: Vec<u64> = value
            .split(&[':', ' '][..]) 
            .filter_map(|v| v.parse::<u64>().ok())
            .collect();

        if nums.len() == 0 {
            return Err(CalibrationError::InvalidString)
        }
        if let Some((result, equation)) = nums.split_first() {
            Ok(Self {
                solution: result.clone(),
                equation: equation.to_vec(),
            })
        } else {
            Err(CalibrationError::InvalidString)
        }
    }

    pub fn is_valid_part1(&self) -> bool {
        let starting_value = self.equation[0];
        return self.calc_part1(1, starting_value)
    }

    pub fn is_valid_part2(&self) -> bool {
        let starting_value = self.equation[0];
        return self.calc_part2(1, starting_value)
    }
    fn calc_part1(&self, idx: usize, acc: u64) -> bool {
        if self.equation.len() == idx && acc == self.solution {
            return true
        }

        if idx >= self.equation.len() {
            return false
        }

        let next_value = if let Some(value) = self.equation.get(idx) {
            value.clone()
        } else {
            return false;
        };

        if acc * next_value <= self.solution && self.calc_part1(idx.add(1), acc * next_value){
            return true
        }

        if acc + next_value <= self.solution && self.calc_part1(idx.add(1), acc + next_value){
            return true
        }
        false
    }

    fn calc_part2(&self, idx: usize, acc: u64) -> bool {
        if idx >= self.equation.len() && acc == self.solution {
            return true
        }

        if idx >= self.equation.len() {
            return false
        }

        let next_value = if let Some(value) = self.equation.get(idx) {
            value.clone()
        } else {
            return false;
        };

        if acc * next_value <= self.solution && self.calc_part2(idx.add(1), acc * next_value){
            return true
        }

        if acc + next_value <= self.solution && self.calc_part2(idx.add(1), acc + next_value){
            return true
        }
        let  new_value = concat_nums(acc, next_value);
        if new_value <= self.solution && self.calc_part2(idx.add(1), new_value){
            return true
        }
        false
    }

}

impl TryFrom<String> for Calibration {
    type Error = CalibrationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Calibration::new(value) {
            Ok(calibration) => Ok(calibration),
            Err(_) => Err(CalibrationError::InvalidString)
        }
    }
}

pub fn concat_nums(num1: u64, num2: u64) -> u64 {
    let num1_as_str = num1.to_string();
    let nums1: Vec<&str> = num1_as_str.split("").collect();

    let num2_as_str = num2.to_string();
    let nums2: Vec<&str> = num2_as_str.split("").collect();

    let mut nums: Vec<&str> = Vec::new();
    for num in nums1 {
        nums.push(num);
    }
    for num in nums2 {
        nums.push(num);
    }

    nums.join("").parse::<u64>().unwrap()
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let file = File::open("data/data.txt")?;
    let reader = BufReader::new(file);
    let mut calibrations: Vec<Calibration> = vec![];
    for line in reader.lines() {
        let line = line?;
        let calibration: Calibration = line.try_into()?;
        calibrations.push(calibration);
    }

    // println!("{:?}", calibrations);

    let start = Instant::now();
    let mut answer_part1 = 0;
    for calibration in &calibrations {
        if calibration.is_valid_part1() {
            answer_part1 += calibration.solution;
        }
    }
    let duration = start.elapsed();
    println!("Answer for Part 1: {}", answer_part1);
    println!("Time for Part 1: {:.2?}", duration);

    let start = Instant::now();
    let mut answer_part2 = 0;
    for calibration in &calibrations {
        if calibration.is_valid_part2() {
            answer_part2 += calibration.solution
        }
    }
    let duration = start.elapsed();
    println!("Answer for Part 2: {}", answer_part2);
    println!("Time for Part 2: {:.2?}", duration);

    Ok(())
}
