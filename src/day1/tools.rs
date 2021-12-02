use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn read_values<T: FromStr>(source: &str) -> Result<Vec<T>> {
    let file = File::open(source)?;
    let lines = io::BufReader::new(file).lines().flatten();

    let mut values: Vec<T> = vec![];
    for line in lines {
        match line.parse::<T>() {
            Ok(num) => values.push(num),
            _ => eprintln!("Error while parsing {}", line),
        }
    }

    Ok(values)
}
