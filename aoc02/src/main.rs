use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn parse_input(path: &Path) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let result = s
        .split(',')
        .map(|num| num.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>();
    result.map_err(|e| e.into())
}

fn run(numbers: &mut [i32], a: i32, b: i32) -> i32 {
    numbers[1] = a;
    numbers[2] = b;
    for i in (0..numbers.len()).step_by(4) {
        match numbers[i] {
            1 => {
                numbers[numbers[i + 3] as usize] =
                    numbers[numbers[i + 1] as usize] + numbers[numbers[i + 2] as usize]
            }
            2 => {
                numbers[numbers[i + 3] as usize] =
                    numbers[numbers[i + 1] as usize] * numbers[numbers[i + 2] as usize]
            }
            _ => break,
        }
    }
    *numbers.first().unwrap()
}

fn part_one(numbers: &[i32]) -> i32 {
    run(&mut numbers.to_owned(), 12, 2)
}

fn part_two(numbers: &[i32]) -> Option<i32> {
    for a in 0..100 {
        for b in 0..100 {
            if run(&mut numbers.to_owned(), a, b) == 19_690_720 {
                return Some(100 * a + b);
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let values = parse_input(Path::new("inputs/input-02.txt"))?;
    println!("Part 1: {}", part_one(&values));
    println!("Part 2: {:?}", part_two(&values));
    Ok(())
}
