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

fn run(arr: &mut [i32], mut phase: Option<i32>, signal: i32, mut i: usize) -> Option<(i32, usize)> {
    loop {
        let (opcode, p1, p2) = (arr[i] % 100, arr[i] / 100 % 10, arr[i] / 1000);
        match opcode {
            99 => return None,
            3 => {
                arr[arr[i + 1] as usize] = if let Some(p) = phase {
                    phase = None;
                    p
                } else {
                    signal
                }
            }
            4 => {
                return Some((
                    arr[if p1 != 0 { i + 1 } else { arr[i + 1] as usize }],
                    i + 2,
                ))
            }
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

fn next_permutation(perm: &mut [i32; 5]) -> Option<()> {
    let i = (0..4).rev().find(|&i| perm[i] < perm[i + 1])?;
    let j = (i..5).rev().find(|&j| perm[j] > perm[i])?;
    perm.swap(i, j);
    perm[i + 1..5].reverse();
    Some(())
}

fn amplify(arr: &[i32], phases: &[i32; 5]) -> i32 {
    let mut signal = 0;
    for &phase in phases {
        signal = run(&mut arr.to_owned(), Some(phase), signal, 0).unwrap().0;
    }
    signal
}

fn part_one(arr: &[i32]) -> i32 {
    let perm = &mut [0, 1, 2, 3, 4];
    let mut max = i32::min_value();
    while next_permutation(perm) != None {
        max = max.max(amplify(arr, perm));
    }
    max
}

fn feedback_loop(arr: &[i32], phases: &[i32; 5]) -> i32 {
    let mut amplifiers = vec![arr.to_owned(); 5];
    let mut indices = [0usize; 5];
    let mut signal = 0;
    for i in 0..5 {
        let pair = run(&mut amplifiers[i], Some(phases[i]), signal, indices[i]).unwrap();
        signal = pair.0;
        indices[i] = pair.1;
    }
    for i in (0..5).cycle() {
        match run(&mut amplifiers[i], None, signal, indices[i]) {
            Some(pair) => {
                signal = pair.0;
                indices[i] = pair.1;
            }
            None => break,
        }
    }
    signal
}

fn part_two(arr: &[i32]) -> i32 {
    let perm = &mut [5, 6, 7, 8, 9];
    let mut max = i32::min_value();
    while next_permutation(perm) != None {
        max = max.max(feedback_loop(arr, perm));
    }
    max
}

fn main() -> Result<(), Box<dyn Error>> {
    let values = parse_input(Path::new("inputs/input-07.txt"))?;
    println!("Part 1: {}", part_one(&values));
    println!("Part 2: {}", part_two(&values));
    Ok(())
}
