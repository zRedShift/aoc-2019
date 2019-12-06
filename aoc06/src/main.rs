use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Orbit {
    parent: Option<String>,
    children: Vec<String>,
}

impl Orbit {
    fn new() -> Orbit {
        Orbit {
            parent: None,
            children: Vec::new(),
        }
    }
}

fn parse_input(path: &Path) -> Result<HashMap<String, Orbit>, Box<dyn Error>> {
    let mut orbits: HashMap<String, Orbit> = HashMap::new();
    for line in BufReader::new(File::open(path)?).lines() {
        let s = &line?;
        let mut iter = s.split(')');
        let parent = iter.next().ok_or("Invalid Input")?.to_string();
        let child = iter.next().ok_or("Invalid Input")?.to_string();
        orbits
            .entry(parent.clone())
            .or_insert_with(Orbit::new)
            .children
            .push(child.clone());
        orbits.entry(child).or_insert_with(Orbit::new).parent = Some(parent)
    }
    if !orbits.contains_key("COM") || !orbits.contains_key("SAN") || !orbits.contains_key("YOU") {
        Err("Invalid Input".into())
    } else {
        Ok(orbits)
    }
}

fn count_orbits(orbits: &HashMap<String, Orbit>, curr: &str, depth: i32) -> i32 {
    orbits[curr].children.iter().fold(depth, |sum, child| {
        sum + count_orbits(orbits, child, depth + 1)
    })
}

fn breadth_first_search(orbits: &HashMap<String, Orbit>) -> Option<i32> {
    let mut queue: VecDeque<(&str, i32)> = VecDeque::new();
    let mut set: HashSet<String> = HashSet::new();
    queue.push_back(("YOU", 0));
    while let Some((curr, depth)) = queue.pop_front() {
        if curr == "SAN" {
            return Some(depth - 2);
        }
        if !set.contains(curr) {
            let orbit = &orbits[curr];
            if let Some(parent) = &orbit.parent {
                queue.push_back((parent, depth + 1));
            }
            for child in orbit.children.iter() {
                queue.push_back((child, depth + 1));
            }
            set.insert(curr.to_string());
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let orbits = parse_input(Path::new("inputs/input-06.txt"))?;
    println!("Part 1: {}", count_orbits(&orbits, "COM", 0));
    println!("Part 1: {:?}", breadth_first_search(&orbits));
    Ok(())
}
