use std::io::{self, BufRead};

mod range;
use range::Range;

fn parse_line(line : &String) -> (Range, Range) {
    let (left, right) = line.split_once(",").expect("No comma?");
    (
        range::from_str(left),
        range::from_str(right)
    )
}

fn main() {

    // Reading
    let mut full_overlap_count: i32 = 0; 
    let stdin : io::Stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        if let Some(line) = lines.next() {
            let line_str = line.expect("Unable to read line");
            let (l, r) = parse_line(&line_str);
            if range::fully_overlap(&l, &r) {
                full_overlap_count+=1;
            }
        }
        else {
            break;
        }
    }

    println!("Full overlap count={}", full_overlap_count);
}
