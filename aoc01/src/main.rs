use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_input(path: &Path) -> Result<Vec<i32>, Box<dyn Error>> {
    let f = File::open(path)?;
    BufReader::new(f)
        .lines()
        .map(|line| {
            line.map_err(|e| e.into())
                .and_then(|s| s.parse::<i32>().map_err(|e| e.into()))
        })
        .collect()
}

fn part_one(numbers: &[i32]) -> i32 {
    numbers.iter().map(|x| x / 3 - 2).sum()
}

fn part_two(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .map(|x| {
            let mut x = x / 3 - 2;
            let mut sum = 0;
            while x >= 0 {
                sum += x;
                x = x / 3 - 2;
            }
            sum
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let values = parse_input(Path::new("inputs/input-01.txt"))?;
    println!("Part 1: {}", part_one(&values));
    println!("Part 2: {}", part_two(&values));
    Ok(())
}
