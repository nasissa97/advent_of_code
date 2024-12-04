use std::fs::File;
use std::io::{self, BufRead, BufReader};

use lazy_regex::regex;
use thiserror::Error;
use core::num::ParseIntError;

pub static DAY3_REGEX: &lazy_regex::Lazy<regex::Regex> = regex!(r"mul\(([1-9]{1}\d{0,2}),([1-9]{1}\d{0,2})\)");

#[derive(Error, Debug)]
pub enum Day3Error {
    #[error("Cannot open file")]
    Io(#[from] io::Error),
    #[error("Invalid Instruction was given")]
    BadInstruction,
    #[error("Failed parsing instruction")]
    BadParse(#[from] ParseIntError),
}

#[derive(Debug)]
struct Instruction {
    x: u64,
    y: u64,
}

impl Instruction {
    pub fn output(&self) -> u64 {
        self.x * self.y
    }
    fn parse_mul(input: &str) -> Result<(u64,u64), Day3Error> {
        let trimmed = &input[4..input.len() - 1];
        let parts: Result<Vec<u64>, Day3Error> = trimmed
            .split(",")
            .into_iter()
            .map(|num| num.parse::<u64>().map_err(|_| Day3Error::BadInstruction))
            .collect();
            
        let nums = match parts {
            Ok(nums) => nums,
            Err(_) => return Err(Day3Error::BadInstruction),
        };    
        
        if nums.len() != 2 {
            return Err(Day3Error::BadInstruction)
        }

        let num1 = nums.get(0).ok_or(Day3Error::BadInstruction)?.clone();
        let num2 = nums.get(1).ok_or(Day3Error::BadInstruction)?.clone();

        Ok((num1, num2))
    }
}

impl TryFrom<String> for Instruction {
    type Error = Day3Error;
    fn try_from(item: String) -> Result<Self, Day3Error> {
        let (x, y) = Instruction::parse_mul(item.as_str())?;
        Ok(Self {x, y})
    }

}


fn main() -> Result<(), Day3Error>{
    println!("Hello, world!");
    let file = File::open("data/data.txt")?;
    let reader = BufReader::new(file);
    // let mut instructions: Vec<Instruction> = Vec::new();
    let mut ans = 0;
    for line in reader.lines() {
        let value = line?;
        // println!("{:?}", value);
        let line_value = DAY3_REGEX.find_iter(&value)
            .map(|match_| match_.as_str().to_string().try_into())
            .filter_map(Result::ok)
            .fold(0, |acc, inst: Instruction| acc + inst.output());

        ans += line_value;
    }

    println!("Answer {}", ans);

    Ok(())
}
