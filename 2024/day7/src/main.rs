use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let file = File::open("data/sample.txt")?;
    let reader = BufReader::new(file);
    let mut calibrations: Vec<Calibration> = vec![];
    for line in reader.lines() {
        let line = line?;
        let calibration: Calibration = line.try_into()?;
        calibrations.push(calibration);
    }

    println!("{:?}", calibrations);

    Ok(())
}
