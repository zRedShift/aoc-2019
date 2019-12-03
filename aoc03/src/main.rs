use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Input = ((i32, i32), i32);

fn parse_helper(s: &str) -> Result<Input, Box<dyn Error>> {
    if s.len() < 2 {
        return Err("".into());
    }
    let d = match s.chars().nth(0).unwrap() {
        'L' => (-1, 0),
        'R' => (1, 0),
        'U' => (0, 1),
        'D' => (0, -1),
        _ => return Err("".into()),
    };
    let num = (s[1..]).parse::<i32>()?;
    Ok((d, num))
}

fn parse_input(path: &Path) -> Result<(Vec<Input>, Vec<Input>), Box<dyn Error>> {
    let f = File::open(path)?;
    let mut b = BufReader::new(f);
    let mut s = String::new();
    b.read_line(&mut s)?;
    let first = s
        .trim_end()
        .split(',')
        .map(parse_helper)
        .collect::<Result<Vec<Input>, _>>()?;
    s.clear();
    b.read_line(&mut s)?;
    let second = s
        .trim_end()
        .split(',')
        .map(parse_helper)
        .collect::<Result<Vec<Input>, _>>()?;
    Ok((first, second))
}

fn run(line: &[Input]) -> HashMap<(i32, i32), i32> {
    let mut position = (0, 0);
    let mut steps = 0;
    let mut grid = HashMap::new();
    for &(direction, length) in line {
        for _ in 0..length {
            position.0 += direction.0;
            position.1 += direction.1;
            steps += 1;
            grid.entry(position).or_insert(steps);
        }
    }
    grid
}

fn parts_one_and_two(line: &[Input], grid: HashMap<(i32, i32), i32>) -> (i32, i32) {
    let mut position = (0, 0);
    let mut steps = 0;
    let mut manhattan = std::i32::MAX;
    let mut step_dist = std::i32::MAX;
    for &(direction, length) in line {
        for _ in 0..length {
            position.0 += direction.0;
            position.1 += direction.1;
            steps += 1;
            if let Some(prev_steps) = grid.get(&position) {
                manhattan = i32::min(manhattan, i32::abs(position.0) + i32::abs(position.1));
                step_dist = i32::min(step_dist, prev_steps + steps);
            }
        }
    }
    (manhattan, step_dist)
}

fn main() -> Result<(), Box<dyn Error>> {
    let values = parse_input(Path::new("inputs/input-03.txt"))?;
    println!("{:?}", parts_one_and_two(&values.1, run(&values.0)));
    Ok(())
}
