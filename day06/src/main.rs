use failure::format_err;
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
    solve.solve2(10000);
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

    /// Solve the second part of the problem.  We're trying to find places
    /// where the sum of the distance to each coord is less than a given
    /// value.  The region given is large (10000), so we have to be a bit
    /// creative with how we determine the search area.
    fn solve2(&self, bound: i32) {

        // To start, let's find the center of all of the coordinates.  This
        // hopefully is within bounds.
        let xsum = self.coords.values().map(|c| c.x as f64).sum::<f64>() /
            (self.coords.len() as f64);
        let ysum = self.coords.values().map(|c| c.y as f64).sum::<f64>() /
            (self.coords.len() as f64);

        let xbase = xsum as i32;
        let ybase = ysum as i32;

        // println!("x: {}, y: {}", xbase, ybase);
        // println!("cum: {}", self.cum_distance(&Coord{x: xbase, y: ybase}));

        let mut total = 0usize;

        // Walk by y coordinates 'up' until we run out of distance.
        for dy in 0.. {
            let y = ybase - dy;

            let sum = self.hwalk(bound, y, xbase);
            if sum == 0 {
                break;
            }

            total += sum;
        }

        for dy in 1.. {
            let y = ybase + dy;

            let sum = self.hwalk(bound, y, xbase);
            if sum == 0 {
                break;
            }

            total += sum;
        }

        println!("Total: {}", total);
    }

    /// Walk across x coordinates determining how many are "inside".
    fn hwalk(&self, bound: i32, y: i32, xbase: i32) -> usize {
        let mut total = 0usize;

        for dx in 0.. {
            let x = xbase - dx;
            let dist = self.cum_distance(&Coord{x: x, y: y});
            if dist >= bound {
                break;
            }

            total += 1;
        }

        for dx in 1.. {
            let x = xbase + dx;
            let dist = self.cum_distance(&Coord{x: x, y: y});
            if dist >= bound {
                break;
            }

            total += 1;
        }

        total
    }

    /// Find the coordinate closest to the given cell.  If it is not
    /// unique, return None.
    fn closest(&self, cell: &Coord) -> Option<usize> {
        let mut unique = false;
        let mut best = None;
        let mut best_value = std::i32::MAX;

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

    /// Find the cumulative distance to all of the coords from the given
    /// point.
    fn cum_distance(&self, cell: &Coord) -> i32 {
        let mut cum = 0;

        for pos in self.coords.values() {
            let dist = (cell.x - pos.x).abs() + (cell.y - pos.y).abs();
            cum += dist;
        }
        cum
    }
}

fn get_input() -> Result<Vec<Coord>> {
    let re = Regex::new(r"^(\d+), (\d+)$")?;
    let f = BufReader::new(File::open("coords.txt")?);

    f.lines().map(|line| {
        let line = line?;

        match re.captures(&line) {
            None => Err(format_err!("Invalid line: {:?}", line)),
            Some(cap) => {
                let x = cap[1].parse().unwrap();
                let y = cap[2].parse().unwrap();
                Ok(Coord{x,y})
            }
        }
    }).collect()
}
