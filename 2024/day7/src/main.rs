use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::time::Instant;

use anyhow::{Context, Result};
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
enum CalibrationError {
    #[error("Invalid format caliberation string")]
    InvalidString,
}

pub struct CalibrationParser;

impl CalibrationParser {
    pub fn parse(value: &str) -> Result<Vec<u64>, CalibrationError> {
        let nums: Vec<u64> = value
            .split(&[':', ' '][..])
            .filter_map(|v| v.parse::<u64>().ok())
            .collect();

        if nums.is_empty() {
            return Err(CalibrationError::InvalidString)
        }
        Ok(nums)
    }   
}

#[derive(Debug, Clone)]
struct Calibration {
    solution: u64,
    equation: Vec<u64>

}

impl Calibration {
    pub fn new(value: &str) -> Result<Self, CalibrationError> {
        let nums = CalibrationParser::parse(value)?;
        let (result, equation) = nums.split_first() 
            .ok_or(CalibrationError::InvalidString)?;

        Ok(Self {
            solution: result.clone(),
            equation: equation.to_vec(),
        })
    }

    pub fn is_valid(&self, strategy: &dyn ValidationStrategy) -> bool {
        strategy.validate(self)
    }
}

pub trait ValidationStrategy: Send + Sync {
    fn validate(&self, calibration: &Calibration) -> bool;
}

pub struct Part1Validation; 

impl ValidationStrategy for Part1Validation {
    fn validate(&self, calibration: &Calibration) -> bool {
        Self::calculate(1, calibration.equation[0], calibration)
    }
}

impl Part1Validation {
    fn calculate(idx: usize, acc: u64, calibration: &Calibration) -> bool {
        if idx == calibration.equation.len() && acc == calibration.solution {
            return true
        }

        if idx >= calibration.equation.len() {
            return false
        }

        let next_value = calibration.equation[idx];

        if acc * next_value <= calibration.solution 
            && Self::calculate(idx + 1, acc * next_value, calibration)
        {
            return true
        }

        if acc + next_value <= calibration.solution 
            && Self::calculate(idx + 1, acc + next_value, calibration)
        {
            return true
        }
        false
    }
}

pub struct Part2Validation;

impl ValidationStrategy for Part2Validation {
    fn validate(&self, calibration: &Calibration) -> bool {
        Self::calculate(1, calibration.equation[0], calibration)
    }
}

impl Part2Validation {
    fn calculate(idx: usize, acc: u64, calibration: &Calibration) -> bool {
        if idx == calibration.equation.len() && acc == calibration.solution {
            return true
        }

        if idx >= calibration.equation.len() {
            return false
        }

        let next_value = calibration.equation[idx];

        if acc * next_value <= calibration.solution 
            && Self::calculate(idx + 1, acc * next_value, calibration)
        {
            return true
        }

        if acc + next_value <= calibration.solution 
            && Self::calculate(idx + 1, acc + next_value, calibration)
        {
            return true
        }

        let  new_value = concat_nums(acc, next_value);
        if new_value <= calibration.solution 
            && Self::calculate(idx.add(1), new_value, calibration)
        {
            return true
        }
        false
    }

}


pub fn concat_nums(num1: u64, num2: u64) -> u64 {
    format!("{}{}", num1, num2).parse::<u64>().unwrap()
}

// Part 1 No Parallelism => 3.89ms
// Part 1 With Parallelism => 904.75us
// Part 2 No Parallelism => 319.06ms
// Part 2 With Parallelism => 52.60ms
fn main() -> Result<()> {
    println!("Hello, world!");
    let file = File::open("data/data.txt")?;
    let reader = BufReader::new(file);
    let calibrations: Vec<Calibration> = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| Calibration::new(&line).ok())
        .collect();


        let start = Instant::now();
        let answer_part1 = calculate_solution(&calibrations, &Part1Validation);
        println!("Answer for Part 1: {}", answer_part1);
        println!("Time for Part 1: {:.2?}", start.elapsed());
    
        let start = Instant::now();
        let answer_part2 = calculate_solution(&calibrations, &Part2Validation);
        println!("Answer for Part 2: {}", answer_part2);
        println!("Time for Part 2: {:.2?}", start.elapsed());
    Ok(())
}

fn calculate_solution(
    calibrations: &[Calibration],
    strategy: &dyn ValidationStrategy,
) -> u64 {
    calibrations
        .par_iter()
        .filter(|calibration| calibration.is_valid(strategy))
        .map(|calibration| calibration.solution)
        .sum()
}
