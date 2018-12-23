use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    result,
};
use chrono::{
    Timelike,
};
use crate::parser::{
    Event,
    Op,
    Parser,
};

mod parser;

// Note that this boxes the error, and requires it to be Send/Sync.
type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut evts = get_input()?;

    evts.sort_by(|x, y| x.time.cmp(&y.time));
    let (a, b) = strat1(&evts);
    println!("1: a={}, b={}, a*b={}", a, b, a * b);
    let (a, b) = strat2(&evts);
    println!("2: a={}, b={}, a*b={}", a, b, a * b);

    Ok(())
}

fn strat1(events: &[Event]) -> (u32, u32) {
    // Current guard.
    let mut current = None;

    // All of the guards.
    let mut all = AllGuards(HashMap::new());

    for ev in events {
        match ev.op {
            Op::Shift(gnum) => {
                current = Some(gnum);
            },
            Op::Sleeps => all.get_mut(current.unwrap()).sleep(ev.time.minute()),
            Op::Wakes => all.get_mut(current.unwrap()).wake(ev.time.minute()),
        }
    }

    // Get all of the guards, and then sort by total time slept.
    let mut all: Vec<_> = all.0.drain().map(|(_, g)| g).collect();
    all.sort_by(|a, b| b.total_sleep.cmp(&a.total_sleep));

    let num = all[0].num;
    let best = all[0].best_minute();

    (num, best)
}

/// For strategy 2, which guard is most frequently asleep on the same
/// minute.
fn strat2(events: &[Event]) -> (u32, u32) {
    // For each minute, maintain a mapping by guard ID to a count for that
    // guard.
    let mut mins: Vec<HashMap<u32, u32>> = vec![HashMap::new(); 60];
    let mut current = 0;
    let mut to_sleep = 0;

    for ev in events {
        match ev.op {
            Op::Shift(gnum) => current = gnum,
            Op::Sleeps => to_sleep = ev.time.minute(),
            Op::Wakes => {
                for min in to_sleep .. ev.time.minute() {
                    *(mins[min as usize].entry(current).or_insert(0)) += 1;
                }
            }
        }
    }

    // Convert each minute to a vector, sorted by the guard with the most
    // sleep first.
    let mins = mins.into_iter().map(|m| {
        let mut mm: Vec<_> = m.iter().map(|(&a, &b)| (a, b)).collect();
        mm.sort_by(|a, b| b.1.cmp(&a.1));
        mm
    });

    // Now convert this to a vector with the index at first, and sort.
    let mins: Vec<_> = mins.into_iter().enumerate().collect();

    // Eliminate any minutes that have no sleeping guards.
    let mut mins: Vec<_> = mins.into_iter().filter(|x| !x.1.is_empty()).collect();
    mins.sort_by(|a, b| b.1[0].1.cmp(&a.1[0].1));

    (mins[0].1[0].0, mins[0].0 as u32)
}

/// All of the guards are kept as a map of this structure.
struct AllGuards(HashMap<u32, Guard>);

impl AllGuards {
    /// Return a mutable reference to the given guard.  A new guard will be
    /// inserted if necessary.
    fn get_mut(&mut self, gnum: u32) -> &mut Guard {
        self.0.entry(gnum).or_insert_with(|| Guard::new(gnum))
    }
}

/// Tracker for an individual guard.
struct Guard {
    num: u32,
    total_sleep: u32,
    slept: Option<u32>,
    minute_count: Vec<u32>,
}

impl Guard {
    fn new(num: u32) -> Guard {
        Guard {
            num: num,
            total_sleep: 0,
            slept: None,
            minute_count: vec![0; 60],
        }
    }

    fn sleep(&mut self, minute: u32) {
        match self.slept {
            None => self.slept = Some(minute),
            Some(_) => panic!("Sleeping guard is sleeping"),
        }
    }

    fn wake(&mut self, minute: u32) {
        match self.slept {
            None => panic!("Awake guard awakes"),
            Some(before) => {
                self.total_sleep += minute - before;
                self.slept = None;
                for i in before .. minute {
                    self.minute_count[i as usize] += 1;
                }
            },
        }
    }

    // Find the minute the guard is most asleep.
    fn best_minute(&self) -> u32 {
        let mut mins: Vec<_> = self.minute_count.iter().cloned().enumerate().collect();
        mins.sort_by(|a, b| b.1.cmp(&a.1));

        println!("mins: {:?}", mins);
        mins[0].0 as u32
    }
}

fn get_input() -> Result<Vec<Event>> {
    let p = Parser::new();
    let f = BufReader::new(File::open("shift.txt")?);

    f.lines().map(|line| {
        let line = line?;

        Ok(p.parse_line(&line)?)
    }).collect()
}
