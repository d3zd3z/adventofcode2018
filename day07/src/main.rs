use failure::format_err;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{
        BinaryHeap,
        BTreeSet,
    },
    fs::File,
    io::{BufRead, BufReader},
    mem,
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let depends = get_input()?;

    let mut t1 = Tracker::new(depends.clone());
    t1.solve1();

    let mut t2 = Tracker::new(depends);
    t2.solve2(5);

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Worker {
    /// Time when this worker is done.
    finish: usize,

    /// What this working is working on.
    work: char,
}

// Implement a reversed Ord as ordered by decreasing time.  (binary_heap is
// a max_queue by default).  Ties are won with ordering (lowest first as
// well) of the working character.  This is needed so that PartialEq is
// consistent with ordering.
impl Ord for Worker {
    fn cmp(&self, other: &Worker) -> Ordering {
        other.finish.cmp(&self.finish)
            .then_with(|| other.work.cmp(&self.work))
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Worker) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Tracker {
    deps: Vec<Depend>,
    todo: BTreeSet<char>,
}

impl Tracker {
    fn new(deps: Vec<Depend>) -> Tracker {
        Tracker {
            deps: deps,
            todo: (b'A' ..= b'Z').map(|c| c as char).collect(),
        }
    }

    fn solve1(&mut self) {
        let mut result = String::new();

        while let Some(ch) = self.best() {
            result.push(ch);
            self.mark_done(ch);
        }

        println!("Result1: {:?}", result);
    }

    fn solve2(&mut self, nworkers: usize) {
        let mut time = 0;
        let mut workers = BinaryHeap::new();

        loop {
            // Fill up any missing work.
            while workers.len() < nworkers {
                if let Some(best) = self.best() {
                    workers.push(Worker{
                        finish: time + (best as usize) - 4,  // A is 65.
                        work: best,
                    });
                    // Remove from todo list early.
                    self.todo.remove(&best);
                } else {
                    break;
                }
            }

            // Take the best work.
            if let Some(dw) = workers.pop() {
                self.mark_done(dw.work);
                time = dw.finish;
            } else {
                break;
            }
        }

        // The result is the timer when we are done.
        println!("Result2: {}", time);
    }

    /// Return the best possible move, with the given dependencies.
    /// Returns None if we are completely done.
    fn best(&self) -> Option<char> {
        let mut todo = self.todo.clone();

        // Remove any that depend on something.
        for dep in &self.deps {
            todo.remove(&dep.post);
        }

        todo.iter().next().cloned()
    }

    /// Mark a given letter as done.
    fn mark_done(&mut self, item: char) {
        self.todo.remove(&item);

        let work = mem::replace(&mut self.deps, vec![]);
        self.deps = work.into_iter().filter(|x| x.pre != item).collect();
    }
}

#[derive(Clone, Debug)]
struct Depend {
    pre: char,
    post: char,
}

fn get_input() -> Result<Vec<Depend>> {
    let re = Regex::new(r"^Step (.) must be finished before step (.) can begin\.$")?;
    let f = BufReader::new(File::open("steps.txt")?);

    f.lines().map(|line| {
        let line = line?;

        match re.captures(&line) {
            None => Err(format_err!("Invalid line: {:?}", line)),
            Some(cap) => {
                // TODO: I don't know why I just can't do `cap[1][0]`.
                let pre = cap.get(1).unwrap().as_str().chars().next().unwrap();
                let post = cap.get(2).unwrap().as_str().chars().next().unwrap();
                Ok(Depend{pre, post})
            }
        }
    }).collect()
}
