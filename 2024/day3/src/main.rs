use std::fs::File;
use std::io::{self, BufRead, BufReader};

use core::num::ParseIntError;
use lazy_regex::regex;
use thiserror::Error;

pub static DAY3_REGEX_PART1: &lazy_regex::Lazy<regex::Regex> =
    regex!(r"mul\(([1-9]{1}\d{0,2}),([1-9]{1}\d{0,2})\)");
pub static DAY3_REGEX_PART2: &lazy_regex::Lazy<regex::Regex> =
    regex!(r"(do\(\)|don't\(\)|mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\))");

pub static TEST: &lazy_regex::Lazy<regex::Regex> =
    regex!(r"don't\(\).*?do\(\)|don't\(\).*");

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
    fn parse_mul(input: &str) -> Option<Self> {
        let trimmed = &input[4..input.len() - 1];
        let parts: Vec<u64> = trimmed
            .split(",")
            .filter_map(|num| num.parse::<u64>().ok())
            .collect();

        if parts.len() != 2 {
            return None
        }


        Some(Self{x: parts[0], y: parts[1]})
    }
}

impl TryFrom<String> for Instruction {
    type Error = Day3Error;
    fn try_from(item: String) -> Result<Self, Day3Error> {
        let instruction = Instruction::parse_mul(item.as_str());
        match instruction {
            Some(instruction) => Ok(instruction),
            _ => Err(Day3Error::BadInstruction)
        }
    }
}

fn part1(line: String) -> u64 {
    let line_value = DAY3_REGEX_PART1
        .find_iter(&line)
        .map(|match_| match_.as_str().to_string().try_into())
        .filter_map(Result::ok)
        .fold(0, |acc, inst: Instruction| acc + inst.output());

    line_value
}

fn part2(line: String, mut enabled: bool) -> (u64, bool) {
    let mut total = 0;

    for capture in DAY3_REGEX_PART2.find_iter(&line) {
        let instr = capture.as_str();
        if instr == "do()" {
            enabled = true;
        } else if instr == "don't()" {
            enabled = false
        } else if instr.starts_with("mul(") {
            if enabled {
                match Instruction::parse_mul(instr) {
                    Some(instruction) => {
                        total += instruction.output()
                    },
                    _ => println!("Bad Instruction {:?}", instr),
                };
            } 
        }
    }
    (total, enabled)
}

fn main() -> Result<(), Day3Error> {
    println!("Hello, world!");
    let file = File::open("data/data2.txt")?;
    let reader = BufReader::new(file);
    let mut ans = 0;
    let mut enabled = true;
    for line in reader.lines() {
        let value = line?;
        let (total, status) = part2(value, enabled);
        enabled = status;
        ans += total;
    }

    println!("Answer {}", ans);

    Ok(())
}
