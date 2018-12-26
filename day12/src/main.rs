use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader},
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut st = State::from_file("real.txt")?;
    for _ in 0 .. 50 {
        st.update();
        // println!("{} {}", st.show(), st.sum());
    }
    // println!("{:?}", st);
    println!("{:?}", st.sum());

    // By printing these out, we determine that by at least 1000, the values
    // have stabilized, and will increment each time by the number of grown
    // plants.  We can extrapolate from there to get the answer for 50
    // billion, but we have to be careful to use 64-bit values.
    for _ in 50 .. 1000 {
        st.update();
    }
    let count = st.grown.len();
    let base = st.sum();
    println!("count: {}, base: {}", count, base);
    println!("{}", (50_000_000_000u64 - 1000) * count as u64 + base as u64);

    // This clearly isn't the intended way of computing this, as this will
    // take decades to compute.
    // The key is to realize that there is a pattern to the numbers, and we
    // can just adjust the numbers for the larger count.
    /*
    for i in 20u64 .. 50000000000 {
        if i % 1000000 == 0 {
            println!("{}: {:?}", i, st);
        }
        st.update();
    }
    println!("{:?}", st.sum());
    */
    Ok(())
}

#[derive(Debug)]
struct State {
    grown: BTreeSet<i32>,
    valids: u32,
}

impl State {
    fn from_file(name: &str) -> Result<State> {
        let f = BufReader::new(File::open(name)?);
        let mut lines = f.lines();

        let initial = lines.next().unwrap()?;
        if !initial.starts_with("initial state: ") {
            panic!("Invalid first line");
        }

        let mut grown = BTreeSet::new();
        for (num, ch) in initial.bytes().skip(15).enumerate() {
            if ch == b'#' {
                grown.insert(num as i32);
            }
        }

        // Skip a line.
        let blank = lines.next().unwrap()?;
        if blank != "" {
            panic!("Unexpected line: {:?}", blank);
        }

        let mut valids = 0;

        for line in lines {
            let line = line?;
            let fields: Vec<_> = line.split(" => ").collect();

            if fields[1] != "#" {
                continue;
            }
            let mut value = 0;
            for b in fields[0].bytes() {
                value <<= 1;
                if b == b'#' {
                    value |= 1;
                }
            }
            // println!("{:?}: 0b{:b}", fields, value);

            valids |= 1 << value;
        }

        Ok(State {
            grown: grown,
            valids: valids,
        })
    }

    /// Update according to the loaded rules.
    fn update(&mut self) {
        let mut new_grown = BTreeSet::new();

        for p in self.full_iter() {
            let mut value = 0;
            for b in -2 ..= 2 {
                value <<= 1;
                if self.grown.contains(&(p + b)) {
                    value |= 1;
                }
            }

            if self.valids & (1 << value) != 0 {
                new_grown.insert(p);
            }
            // println!("Check at: {:3}: {:05b} {}", p, value, self.valids & (1 << value) != 0);
        }
        // println!("{:?}", new_grown);
        self.grown = new_grown;
    }

    fn sum(&self) -> i32 {
        self.grown.iter().sum()
    }

    #[allow(dead_code)]
    fn show(&self) -> String {
        let mut result = String::new();
        let left = self.grown.iter().next().cloned().unwrap();
        {
            use std::fmt::Write;
            write!(&mut result, "{:6}: ", left).unwrap();
        }
        for p in self.full_iter() {
            result.push(if self.grown.contains(&p) { '#' } else { '.' });
        }
        result
    }

    /// Return an iterator covering the range 2 beyond the maximum/minimum
    /// elements set.
    fn full_iter(&self) -> impl Iterator<Item=i32> {
        let left = self.grown.iter().next().cloned().unwrap();
        let right = self.grown.iter().next_back().cloned().unwrap();
        (left - 2) ..= (right + 2)
    }
}
