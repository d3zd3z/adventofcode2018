//! Parse the journal.

use chrono::naive::{
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
};
use crate::Result;
use regex::Regex;

/// The event we care about:
#[derive(Debug)]
pub struct Event {
    pub time: NaiveDateTime,
    pub op: Op,
}

/// The operation type.
#[derive(Debug)]
pub enum Op {
    Wakes,
    Sleeps,
    Shift(u32)
}

pub struct Parser {
    time_re: Regex,
    shift_re: Regex,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            time_re: Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (.*)$").unwrap(),
            shift_re: Regex::new(r"^Guard #(\d+) begins shift$").unwrap(),
        }
    }

    pub fn parse_line(&self, line: &str) -> Result<Event> {
        let (dt, line) = match self.time_re.captures(line) {
            None => panic!("Invalid line"),
            Some(cap) => {
                let date = NaiveDate::from_ymd(cap[1].parse().unwrap(),
                                               cap[2].parse().unwrap(),
                                               cap[3].parse().unwrap());
                let time = NaiveTime::from_hms(cap[4].parse().unwrap(),
                                               cap[5].parse().unwrap(),
                                               0);
                (NaiveDateTime::new(date, time), cap[6].to_string())
            }
        };

        let op = match &line[..] {
            "wakes up" => Op::Wakes,
            "falls asleep" => Op::Sleeps,
            text => {
                match self.shift_re.captures(text) {
                    None => panic!("Invalid line {:?}", text),
                    Some(cap) => Op::Shift(cap[1].parse().unwrap()),
                }
            }
        };

        Ok(Event {
            time: dt,
            op: op,
        })
    }
}

/*
named!(parse_line(CompleteByteSlice) -> (),
    do_parse!(
        tag!("[") >>
        stamp: timestamp >>
        tag!("] ") >>
        cmd: command >>
        eof!() >>
        (())
    ));

named!(timestamp(CompleteByteSlice) -> NaiveDateTime,
    do_parse!(
        year: map_res!(take_while_m_n!(4, 4, is_digit), from_decimal) >>
        tag!("-") >>
        month: map_res!(take_while_m_n!(2, 2, is_digit), from_decimal) >>
        tag!("-") >>
        day: map_res!(take_while_m_n!(2, 2, is_digit), from_decimal) >>
        tag!(" ") >>
        hour: map_res!(take_while_m_n!(2, 2, is_digit), from_decimal) >>
        tag!(":") >>
        minute: map_res!(take_while_m_n!(2, 2, is_digit), from_decimal) >>
        ({
            let d = NaiveDate::from_ymd(year as i32, month, day);
            let t = NaiveTime::from_hms(hour, minute, 0);
            NaiveDateTime::new(d, t)
        })));

named!(command(CompleteByteSlice) -> Op,
    do_parse!(
       result: switch!(
            tag!("wakes up") => value!(Op::Wakes) |
            tag!("falls asleep") => value!(Op::Sleeps) |
            _ => begin_shift) >>
        (result)
        ));

named!(begin_shift(CompleteByteSlice) -> Op,
    do_parse!(
        tag!("Guard #") >>
        gnum: map_res!(digit1, from_decimal) >>
        tag!(" begins shift") >>
        (Op::Shift(gnum))
    ));

fn from_decimal(input: CompleteByteSlice) -> Result<u32> {
    Ok(u32::from_str_radix(std::str::from_utf8(input.0)?, 10)?)
}
*/
