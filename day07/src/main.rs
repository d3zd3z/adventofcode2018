use regex::Regex;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    mem,
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut depends = get_input()?;

    let mut t1 = Tracker::new(depends.clone());
    t1.solve1();

    Ok(())
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

    let mut result = vec![];
    for line in f.lines() {
        let line = line?;

        match re.captures(&line) {
            None => panic!("Invalid line"),
            Some(cap) => {
                // TODO: I don't know why I just can't do `cap[1][0]`.
                let pre = cap.get(1).unwrap().as_str().chars().next().unwrap();
                let post = cap.get(2).unwrap().as_str().chars().next().unwrap();
                result.push(Depend{pre, post});
            }
        }
    }
    Ok(result)
}
