use failure::format_err;
use regex::Regex;
use std::{
    borrow::Cow,
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let rem = Remover::new();

    let work = get_input()?;
    println!("work: {}", rem.remove(&work).len());

    // Try replacing each character.
    let mut best = work.len();
    for ch in b'a' ..= b'z' {
        let w1 = work.replace(ch as char, "");
        let w2 = w1.replace((ch - 32) as char, "");
        let this_len = rem.remove(&w2).len();
        best = best.min(this_len);
        println!("{}: {}", ch as char, rem.remove(&w2).len());
    }
    println!("Best: {}", best);
    Ok(())
}

struct Remover {
    sub_re: Regex,
}

impl Remover {
    fn new() -> Remover {
        // Build a regex up to eliminate the interesting pairs.
        // Rust gets annoying here because of Unicode.  We'll just do this with
        // bytes.
        let mut pattern = String::new();
        pattern.push('(');
        for ch in b'a' ..= b'z' {
            if pattern.len() > 1 {
                pattern.push('|');
            }
            pattern.push(ch as char);
            pattern.push((ch - 32) as char);
            pattern.push('|');
            pattern.push((ch - 32) as char);
            pattern.push(ch as char);
        }
        pattern.push(')');
        println!("pattern: {:?}", pattern);

        Remover {
            sub_re: Regex::new(&pattern).unwrap(),
        }
    }

    fn remove(&self, text: &str) -> String {
        let mut work = text.to_owned();
        loop {
            // It should be the same whether the replacements happen at the
            // beginning, or make as many passes through the string as we can.
            match self.sub_re.replace_all(&work, "") {
                // It returns an owned string if it made any changes.
                Cow::Owned(txt) => work = txt,
                Cow::Borrowed(_) => break,
            }
        }
        work
    }
}

/// Read the input (which is on a single line).
fn get_input() -> Result<String> {
    let f = BufReader::new(File::open("polymer.txt")?);

    let line = match f.lines().next() {
        Some(l) => l?,
        None => return Err(format_err!("Unable to read polymer")),
    };
    Ok(line)
}
