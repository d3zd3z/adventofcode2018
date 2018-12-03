use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let lines = get_input()?;
    println!("boxes: {}", lines.len());
    println!("Chksum: {}", checksum(&lines));

    Ok(())
}

fn checksum(ids: &[String]) -> u32 {
    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        let mut this_two = 0;
        let mut this_three = 0;
        let mut sorted: Vec<char> = id.chars().collect();
        sorted.sort();

        for (_, grp) in &sorted.into_iter().group_by(|&x| x) {
            match grp.count() {
                2 => this_two = 1,
                3 => this_three = 1,
                _ => (),
            }
        }
        twos += this_two;
        threes += this_three;
    }

    twos * threes
}

/// Read all of the lines from the input file.
fn get_input() -> Result<Vec<String>> {
    let f = BufReader::new(File::open("ids.txt")?);

    let mut result = vec![];
    for line in f.lines() {
        let line = line?;
        result.push(line);
    }
    Ok(result)
}
