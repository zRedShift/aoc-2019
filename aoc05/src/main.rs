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

fn run(arr: &mut [i32], input: i32) {
    let mut i = 0;
    loop {
        let (opcode, p1, p2) = (arr[i] % 100, arr[i] / 100 % 10, arr[i] / 1000);
        match opcode {
            99 => return,
            3 => arr[arr[i + 1] as usize] = input,
            4 => println!("{}", arr[if p1 != 0 { i + 1 } else { arr[i + 1] as usize }]),
            _ => {
                let val1 = arr[if p1 != 0 { i + 1 } else { arr[i + 1] as usize }];
                let val2 = arr[if p2 != 0 { i + 2 } else { arr[i + 2] as usize }];
                match opcode {
                    1 => arr[arr[i + 3] as usize] = val1 + val2,
                    2 => arr[arr[i + 3] as usize] = val1 * val2,
                    5 => i = if val1 != 0 { val2 as usize } else { i + 3 },
                    6 => i = if val1 == 0 { val2 as usize } else { i + 3 },
                    7 => arr[arr[i + 3] as usize] = (val1 < val2) as i32,
                    8 => arr[arr[i + 3] as usize] = (val1 == val2) as i32,
                    _ => unreachable!(),
                }
            }
        }
        i += [0, 4, 4, 2, 2, 0, 0, 4, 4][opcode as usize];
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
