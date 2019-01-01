use failure::format_err;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use regex::Regex;
use std::{
    error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    result,
};

type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let input = Input::from_file("input.txt")?;
    // println!("Input: {:?}", input);

    let mut total_count = 0;
    for sample in &input.samples {
        // println!("Trying: {:?}", sample);
        let mut count = 0;
        for op in Opcode::iter() {
            let mut reg = sample.before.clone();
            op.eval(&sample.op, &mut reg);
            // println!("  {:?}: {:?}", op, reg);
            if reg == sample.after {
                count += 1;
            }
        }
        // println!("{} matched", count);
        if count >= 3 {
            total_count += 1;
        }
    }
    println!("Total count: {}", total_count);
    Ok(())
}

#[derive(Debug)]
struct Sample {
    before: [u8; 4],
    op: [u8; 4],
    after: [u8; 4],
}

#[derive(Debug)]
struct Input {
    samples: Vec<Sample>,
}

/// The opcodes given.  The order listed here is just given in the problem
/// description.  To solve, we will determine a mapping from the opcode
/// numbers to these enums.
impl Input {
    fn from_file<P: AsRef<Path>>(name: P) -> Result<Input> {
        let before_re = Regex::new(r"^Before: \[(\d), (\d), (\d), (\d)\]$")?;
        let after_re = Regex::new(r"^After:  \[(\d), (\d), (\d), (\d)\]$")?;
        let op_re = Regex::new(r"^(\d+) (\d) (\d) (\d)$")?;

        let f = BufReader::new(File::open(name)?);

        let mut lines = f.lines();
        let mut samples = vec![];

        loop {
            let before = match scan_line(&mut lines, &before_re)? {
                Some(b) => b,
                None => break,
            };
            let op = scan_line(&mut lines, &op_re)?;
            let after = scan_line(&mut lines, &after_re)?;
            scan_blank(&mut lines)?;

            samples.push(Sample {
                before: before,
                op: op.unwrap(),
                after: after.unwrap(),
            });
        }

        // TODO: Read additional lines.

        Ok(Input{
            samples: samples,
        })
    }
}

/// Attempt to read a line from the input, matching it against the given
/// regex, and returning the four values in an array.
fn scan_line<I, E: 'static>(rd: &mut I, re: &Regex) -> Result<Option<[u8; 4]>>
    where I: Iterator<Item = result::Result<String, E>>,
          E: error::Error+Send+Sync,
{
    if let Some(line) = rd.next() {
        let line = line?;

        match re.captures(&line) {
            // None => Err(format_err!("Unmatched line: {:?} ({:?}", line, re)),
            None => Ok(None),
            Some(cap) => {
                let p1 = cap[1].parse().unwrap();
                let p2 = cap[2].parse().unwrap();
                let p3 = cap[3].parse().unwrap();
                let p4 = cap[4].parse().unwrap();
                Ok(Some([p1, p2, p3, p4]))
            }
        }
    } else {
        Ok(None)
    }
}

/// Make sure the next line read is blank.
fn scan_blank<I, E: 'static>(rd: &mut I) -> Result<()>
    where I: Iterator<Item = result::Result<String, E>>,
          E: error::Error+Send+Sync,
{
    if let Some(line) = rd.next() {
        let line = line?;

        if !line.is_empty() {
            Err(format_err!("Expecting blank line: {:?}", line))
        } else {
            Ok(())
        }
    } else {
        Err(format_err!("Not expecting eof, expecting blank line"))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

/// Operation modes.  The set instructions ignore the second argument, but
/// it will always be ok to just use one of the modes (we'll use Reg just
/// to avoid needing an ImmImm mode that only ignores the second arg).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    RegReg,
    ImmReg,
    RegImm
}

/// Modes.
static OP_MODE: [Mode; 16] = [
    Mode::RegReg, // Addr
    Mode::RegImm, // Addi
    Mode::RegReg, // Mulr
    Mode::RegImm, // Muli
    Mode::RegReg, // Banr
    Mode::RegImm, // Bani
    Mode::RegReg, // Borr
    Mode::RegImm, // Bori
    Mode::RegReg, // Setr
    Mode::ImmReg, // Seti
    Mode::ImmReg, // Gtir
    Mode::RegImm, // Gtri
    Mode::RegReg, // Gtrr
    Mode::ImmReg, // Eqir
    Mode::RegImm, // Eqri
    Mode::RegReg, // Eqrr
];

/// Operations themselves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
    Ban,
    Bor,
    Set,
    Gt,
    Eq,
}

/// The operations.
static OP_OPERATION: [Operation; 16] = [
    Operation::Add, // Addr
    Operation::Add, // Addi
    Operation::Mul, // Mulr
    Operation::Mul, // Muli
    Operation::Ban, // Banr
    Operation::Ban, // Bani
    Operation::Bor, // Borr
    Operation::Bor, // Bori
    Operation::Set, // Setr
    Operation::Set, // Seti
    Operation::Gt, // Gtir
    Operation::Gt, // Gtri
    Operation::Gt, // Gtrr
    Operation::Eq, // Eqir
    Operation::Eq, // Eqri
    Operation::Eq, // Eqrr
];

impl Opcode {
    /// Simulate a given operation.  Although the instruction as the opcode
    /// as its first element, this uses the 'self' argument so that the
    /// clients can try different opcodes.
    fn eval(&self, instr: &[u8; 4], regs: &mut [u8; 4]) {
        let (a, b) = match OP_MODE[*self as usize] {
            Mode::RegReg => (regs[instr[1] as usize], regs[instr[2] as usize]),
            Mode::ImmReg => (instr[1], regs[instr[2] as usize]),
            Mode::RegImm => (regs[instr[1] as usize], instr[2]),
        };

        let c = match OP_OPERATION[*self as usize] {
            Operation::Add => a + b,
            Operation::Mul => a * b,
            Operation::Ban => a & b,
            Operation::Bor => a | b,
            Operation::Set => a,
            Operation::Gt => if a > b { 1 } else { 0 },
            Operation::Eq => if a == b { 1 } else { 0 },
        };

        regs[instr[3] as usize] = c;
    }

    /// Return an iterator over all of the opcodes.
    fn iter() -> OpcodeIter {
        OpcodeIter(0)
    }
}

struct OpcodeIter(usize);

impl Iterator for OpcodeIter {
    type Item = Opcode;

    fn next(&mut self) -> Option<Opcode> {
        let cur = self.0;

        if cur < 16 {
            self.0 += 1;
            Some(FromPrimitive::from_usize(cur).unwrap())
        } else {
            None
        }
    }
}
