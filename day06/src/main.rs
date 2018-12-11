use regex::Regex;
use std::{
    collections::{
        BTreeMap,
        HashMap,
        HashSet,
    },
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let work = get_input()?;
    let solve = Solver::new(&work);
    // println!("solve: {:?}", solve);

    solve.solve1();
    Ok(())
}

#[derive(Debug, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Solver {
    // Bounds of the problem space.
    min: Coord,
    max: Coord,

    // The coordinates themselves.  This is a map, just to give the
    // coordinates a label.
    coords: BTreeMap<usize, Coord>,
}

impl Solver {
    fn new(coords: &[Coord]) -> Solver {
        let min_x = coords.iter().map(|c| c.x).min().unwrap();
        let max_x = coords.iter().map(|c| c.x).max().unwrap();
        let min_y = coords.iter().map(|c| c.y).min().unwrap();
        let max_y = coords.iter().map(|c| c.y).max().unwrap();
        Solver {
            min: Coord{x: min_x, y: min_y},
            max: Coord{x: max_x, y: max_y},
            coords: coords.iter().cloned().enumerate().collect(),
        }
    }

    /// Solve the first part of the problem.  We search for the area around
    /// each coordinate that is closest to that coordinate.  Don't count
    /// any cells that are equidistant to two coordinates.  In addition,
    /// discard any coordinate that has a nearest cell on the outer
    /// boundary, as these will be unbounded.
    fn solve1(&self) {
        // Ones we find on the edge will be discarded here.
        let mut discards: HashSet<usize> = HashSet::new();

        // Mapping between coordinate indices and the count of how many
        // cells have been seen.
        let mut counts: HashMap<usize, usize> = HashMap::new();

        for y in self.min.y ..= self.max.y {
            for x in self.min.x ..= self.max.x {
                match self.closest(&Coord{x, y}) {
                    None => (),
                    Some(cell) => {
                        if x == self.min.x || x == self.max.x || y == self.min.y || y == self.max.y {
                            // This is on edge, ignore this cell entirely.
                            discards.insert(cell);
                        } else {
                            *counts.entry(cell).or_insert(0) += 1;
                        }
                    }
                }
                // println!("({},{}) = {:?}", x, y, closest);
            }
        }

        for d in discards.into_iter() {
            counts.remove(&d);
        }

        // println!("{:?}", counts);

        // The result is the cell with the largest count.
        match counts.iter().max_by_key(|c| c.1) {
            Some((_, count)) => println!("result1: {}", count),
            None => println!("No result"),
        }
    }

    /// Find the coordinate closest to the given cell.  If it is not
    /// unique, return None.
    fn closest(&self, cell: &Coord) -> Option<usize> {
        let mut unique = false;
        let mut best = None;
        let mut best_value = 12345678; // Tacky, best to use max value for int.

        for (&k, pos) in self.coords.iter() {
            let dist = (cell.x - pos.x).abs() + (cell.y - pos.y).abs();
            if dist < best_value {
                unique = true;
                best = Some(k);
                best_value = dist;
            } else if dist == best_value {
                unique = false;
            }
        }

        if unique {
            Some(best.unwrap())
        } else {
            None
        }
    }
}

fn get_input() -> Result<Vec<Coord>> {
    let re = Regex::new(r"^(\d+), (\d+)$")?;
    let f = BufReader::new(File::open("coords.txt")?);

    let mut result = vec![];
    for line in f.lines() {
        let line = line?;

        match re.captures(&line) {
            None => panic!("Invalid line"),
            Some(cap) => {
                let x = cap[1].parse().unwrap();
                let y = cap[2].parse().unwrap();
                result.push(Coord{x,y});
            }
        }
    }
    Ok(result)
}
