use std::{
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let lines = get_input()?;

    'outside:
    for outer in 0 .. lines.len() {
        for inner in outer + 1 .. lines.len() {
            if delta(&lines[outer], &lines[inner]) == 1 {
                show(&lines[outer], &lines[inner]);
                break 'outside;
            }
        }
    }

    Ok(())
}

fn delta(a: &str, b: &str) -> u32 {
    a.chars().zip(b.chars()).map(|(aa, bb)| {
        if aa == bb {
            0
        } else {
            1
        }
    }).sum()
}

fn show(a: &str, b: &str) {
    let mut result = String::new();

    for (aa, bb) in a.chars().zip(b.chars()) {
        if aa == bb {
            result.push(aa);
        }
    }
    println!("{:?}", result);
}

/// Read all of the lines from the input file.
fn get_input() -> Result<Vec<String>> {
    let f = BufReader::new(File::open("ids.txt")?);

    f.lines().map(|line| Ok(line?)).collect()
}
