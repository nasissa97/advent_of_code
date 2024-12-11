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

    pub fn is_valid(&self) -> bool {
        let starting_value = self.equation[0];
        return self.calc(1, starting_value)
    }
    fn calc(&self, idx: usize, acc: u64) -> bool {
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

        if acc * next_value <= self.solution && self.calc(idx.add(1), acc * next_value){
            return true
        }

        if acc + next_value <= self.solution && self.calc(idx.add(1), acc + next_value){
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
    for calibration in calibrations {
        if calibration.is_valid() {
            answer_part1 += calibration.solution;
        }
    }
    let duration = start.elapsed();
    println!("Answer for Part 1: {}", answer_part1);
    println!("Time for Part 1: {:.2?}", duration);

    Ok(())
}
