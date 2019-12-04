use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;
use std::path::Path;

fn parse_input(path: &Path) -> Result<(RangeInclusive<i32>), Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let mut iter = s.split('-');
    Ok(iter.next().ok_or("")?.parse::<i32>()?..=(iter.next().ok_or("")?.parse::<i32>()?))
}

fn parts_one_and_two(range: RangeInclusive<i32>) -> (i32, i32) {
    let mut sums = (0, 0);
    for mut x in range {
        let mut matches = (0, 0);
        let mut prev = 10;
        let mut count = 0;
        while x != 0 {
            let curr = x % 10;
            if curr > prev {
                matches = (0, 0);
                count = 0;
                break;
            }
            if curr == prev {
                count += 1;
                matches.0 = 1;
            } else if count == 1 {
                matches.1 = 1;
            } else {
                count = 0;
            }
            prev = curr;
            x /= 10;
        }
        if count == 1 {
            matches.1 = 1;
        }
        sums.0 += matches.0;
        sums.1 += matches.1;
    }
    sums
}

fn main() -> Result<(), Box<dyn Error>> {
    let range = parse_input(Path::new("inputs/input-04.txt"))?;
    println!("{:?}", parts_one_and_two(range));
    Ok(())
}
