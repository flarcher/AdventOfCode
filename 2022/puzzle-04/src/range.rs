use std::{fmt::Display};

#[derive(Debug)]
pub struct Range {
    min : u8,
    max : u8
}

impl Range {

    pub fn min(&self) -> u8 {
        self.min
    }

    pub fn max(&self) -> u8 {
        self.max
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    pub fn has(&self, val: &u8) -> bool {
        self.min >= *val && *val <= self.max
    }
}

pub fn fully_overlap(left: &Range, right: &Range) -> bool {
    left.contains(right) || right.contains(left)
}

pub fn overlap(left: &Range, right: &Range) -> bool {
    if left.max < right.min || right.max < left.min {
        return false
    }
    if fully_overlap(left, right) {
        return true
    }
    let real_min = std::cmp::min(left.min, right.min);
    let real_max = std::cmp::max(left.max, right.max);
    for i in real_min..real_max {
        if left.has(&i) && right.has(&i) {
            return true
        }
    }
    false
}

pub fn from_str(string : &str) -> Range {
    let (min, max) = string.split_once("-").expect("No dash?");
    Range {
        min: u8::from_str_radix(min, 10).expect("Parse error"),
        max: u8::from_str_radix(max, 10).expect("Parse error")
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.min, self.max))
    }
}