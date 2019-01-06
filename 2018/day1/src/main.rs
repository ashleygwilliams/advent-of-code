use std::collections::HashSet;
use std::fs;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let input: String = fs::read_to_string("./input.txt")?;
    sum_frequencies(&input)?;
    duplicate_frequency(&input)?;
    Ok(())
}

fn sum_frequencies(input: &str) -> Result<()> {
    let mut frequency = 0;
    for line in input.lines() {
        let change: i32 = line.parse()?;
        frequency += change;
    }
    println!("PART 1 SOLUTION: {}", frequency);
    Ok(())
}

fn duplicate_frequency(input: &str) -> Result<()> {
    let mut frequency = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(0);

    loop {
        for line in input.lines() {
            let change: i32 = line.parse()?;
            frequency += change;
            if frequencies.contains(&frequency) {
                println!("PART 2 SOLUTION: {}", frequency);
                return Ok(());
            }
            frequencies.insert(frequency);
        }
    }
}
