use std::collections::HashMap;
use std::fs;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let input: String = fs::read_to_string("./input.txt")?;
    let maps = charmaps(&input);
    let mut count2 = 0;
    let mut count3 = 0;

    for id in maps {
        if count_freq(&id, 2)? {
            count2 += 1;
        }
        if count_freq(&id, 3)? {
            count3 += 1;
        }
    }

    println!("SOLUTION 1: {}", count2 * count3);
    Ok(())
}

fn charmaps(input: &str) -> Vec<HashMap<char, i32>> {
    let mut maps = vec![];
    for line in input.lines() {
        maps.push(charmap(line));
    }
    maps
}

fn charmap(s: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }
    map
}

fn count_freq(m: &HashMap<char, i32>, target: i32) -> Result<bool> {
    for c in m.keys() {
        if m.get(c).unwrap() == &target {
            return Ok(true);
        }
    }
    Ok(false)
}
