// use image::ColorType;
use failure::format_err;
use regex::Regex;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut points = get_input()?;

    // Keep adjusting the image until the size stops shrinking.
    let mut last_size = std::isize::MAX;
    let mut count = 0;
    loop {
        adjust(&mut points);
        let nsize = size(&points);
        if nsize > last_size {
            unadjust(&mut points);
            break;
        }
        last_size = nsize;
        count += 1
    }
    println!("size: {:?}", last_size);
    println!("count: {:?}", count);
    save(&points)?;

    Ok(())
}

fn size(points: &[Light]) -> isize {
    let minx = points.iter().map(|p| p.x).min().unwrap();
    let maxx = points.iter().map(|p| p.x).max().unwrap();
    let miny = points.iter().map(|p| p.y).min().unwrap();
    let maxy = points.iter().map(|p| p.y).max().unwrap();

    (maxy - miny).max(maxx - minx)
}

fn save(points: &[Light]) -> Result<()> {
    let minx = points.iter().map(|p| p.x).min().unwrap();
    let maxx = points.iter().map(|p| p.x).max().unwrap();
    let miny = points.iter().map(|p| p.y).min().unwrap();
    let maxy = points.iter().map(|p| p.y).max().unwrap();

    let kept: BTreeSet<(isize, isize)> = points.iter().map(|p| (p.x, p.y)).collect();

    for y in miny ..= maxy {
        for x in minx ..= maxx {
            print!("{}", if kept.contains(&(x, y)) { '*' } else { ' ' });
        }
        println!("");
    }
    Ok(())
}

/// Adjust the points for the given movement.
fn adjust(points: &mut [Light]) {
    for mut p in points {
        p.x += p.dx;
        p.y += p.dy;
    }
}

/// Back out the last adjustment.
fn unadjust(points: &mut [Light]) {
    for mut p in points {
        p.x -= p.dx;
        p.y -= p.dy;
    }
}

#[derive(Clone, Debug)]
struct Light {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

fn get_input() -> Result<Vec<Light>> {
    let re = Regex::new(r"^position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>$")?;
    let f = BufReader::new(File::open("lights.txt")?);

    f.lines().map(|line| {
        let line = line?;

        match re.captures(&line) {
            None => Err(format_err!("Invalid line: {:?}", line)),
            Some(cap) => {
                let x = cap[1].parse().unwrap();
                let y = cap[2].parse().unwrap();
                let dx = cap[3].parse().unwrap();
                let dy = cap[4].parse().unwrap();
                Ok(Light{x, y, dx, dy})
            }
        }
    }).collect()
}
