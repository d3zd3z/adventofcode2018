// TODO: Narrow this down to what we use.
use failure::format_err;
use nom::{
    named,
    call, eof,
    do_parse, tag, take_while1,
    map_res, error_position,
    space, is_digit,
    types::CompleteByteSlice,
};

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let cuts = get_input()?;

    // This set tracks all of the squares that have been visited.
    let mut visited = HashSet::new();

    // This set tracks the squares that have been visited multiple times.
    let mut multiples = HashSet::new();

    for cut in &cuts {
        for y in cut.y .. cut.y + cut.h {
            for x in cut.x .. cut.x + cut.w {
                if visited.contains(&(x, y)) {
                    multiples.insert((x, y));
                } else {
                    visited.insert((x, y));
                }
            }
        }
    }
    println!("overlaps: {}", multiples.len());

    // To solve the second part, revisit the cuts, and find one that never
    // hits multiples.
    for cut in &cuts {
        let mut hit = false;
        for y in cut.y .. cut.y + cut.h {
            for x in cut.x .. cut.x + cut.w {
                if multiples.contains(&(x, y)) {
                    hit = true;
                }
            }
        }
        if !hit {
            println!("No overlap: {}", cut.num);
        }
    }

    Ok(())
}

fn get_input() -> Result<Vec<Pos>> {
    let f = BufReader::new(File::open("cuts.txt")?);

    f.lines().map(|line| {
        let line = line?;

        // This is a little tricky, because the error back from the parser
        // has a lifetime dependency on the parser input, but the error
        // will need to outlive the input.  To keep things simple, just
        // format the error into a string, and use that.
        let (_, pos) = parse_cut(CompleteByteSlice(line.as_bytes()))
            .map_err(|e| format_err!("Parse error: {:?}", e))?;
        Ok(pos)
    }).collect()
}

#[derive(Debug)]
struct Pos {
    num: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

// Parser for the cuts.
named!(parse_cut(CompleteByteSlice) -> Pos,
    do_parse!(
        tag!("#") >>
        num: decimal >> space >>
        tag!("@") >> space >>
        x: decimal >> tag!(",") >>
        y: decimal >> tag!(":") >> space >>
        w: decimal >> tag!("x") >>
        h: decimal >> eof!() >>
        (
            Pos {
                num: num,
                x: x,
                y: y,
                w: w,
                h: h,
            }
        )
    ));

named!(decimal(CompleteByteSlice) -> i32,
       map_res!(take_while1!(is_digit), from_decimal));

fn from_decimal(input: CompleteByteSlice) -> Result<i32> {
    Ok(i32::from_str_radix(std::str::from_utf8(input.0)?, 10)?)
}
