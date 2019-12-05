use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn parse_input(path: &Path) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.split(',')
        .map(|num| num.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| e.into())
}

fn run(numbers: &mut [i32], input: i32) {
    let mut i = 0;
    loop {
        let (opcode, p1, p2) = (numbers[i] % 100, numbers[i] / 100 % 10, numbers[i] / 1000);
        match opcode {
            99 => break,
            3 => {
                numbers[numbers[i + 1] as usize] = input;
                i += 2;
            }
            4 => {
                println!(
                    "{}",
                    if p1 != 0 {
                        numbers[i + 1]
                    } else {
                        numbers[numbers[i + 1] as usize]
                    }
                );
                i += 2;
            }
            _ => {
                let val1 = if p1 != 0 {
                    numbers[i + 1]
                } else {
                    numbers[numbers[i + 1] as usize]
                };
                let val2 = if p2 != 0 {
                    numbers[i + 2]
                } else {
                    numbers[numbers[i + 2] as usize]
                };
                match opcode {
                    1 => {
                        numbers[numbers[i + 3] as usize] = val1 + val2;
                        i += 4;
                    }
                    2 => {
                        numbers[numbers[i + 3] as usize] = val1 * val2;
                        i += 4;
                    }
                    5 => i = if val1 != 0 { val2 as usize } else { i + 3 },
                    6 => i = if val1 == 0 { val2 as usize } else { i + 3 },
                    7 => {
                        numbers[numbers[i + 3] as usize] = if val1 < val2 { 1 } else { 0 };
                        i += 4;
                    }
                    8 => {
                        numbers[numbers[i + 3] as usize] = if val1 == val2 { 1 } else { 0 };
                        i += 4;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let values = parse_input(Path::new("inputs/input-05.txt"))?;
    println!("Part 1:");
    run(&mut values.to_owned(), 1);
    println!("Part 2:");
    run(&mut values.to_owned(), 5);
    Ok(())
}
