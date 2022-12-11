use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::{env, usize};
use std::collections::VecDeque;

fn parse_stack_symbol(symbol: &str) -> Option<&str> {
    let val = symbol.as_bytes()[0];
    if val.is_ascii_uppercase() { Some(symbol) } else { None }
}

fn parse_stack_info(line : &String, count: usize) -> Vec<Option<String>> {
    let mut result: Vec<Option<String>> = vec![Option::None; count];
    for stack in 0..count {
        let part_index = stack * 4;
        let opt_value = parse_stack_symbol(&line[part_index+1..part_index+2]);
        match opt_value {
            Some(v) => { result[stack] = Some(String::from_str(v).expect("Ouch")) }
            None => ()
        }
    }
    result
}

fn process_stack_info(stacks: &mut Vec<VecDeque<String>>, input: &Vec<Option<String>>) -> () {
    for (index, stack) in input.iter().enumerate() {
        match stack {
            None => (),
            Some(item) => stacks[index].push_back(item.clone())
        }
    }
}

// move q from f to t
fn parse_move_order(line: &String) -> (u8, u8, u8) {
    assert!(line.len() >= 18);
    let tokens : Vec<&str> = line.split_ascii_whitespace().collect();
    assert_eq!(6, tokens.len());
    assert_eq!("move", tokens[0]);
    let quantity = tokens[1];
    assert_eq!("from", tokens[2]);
    let source   = tokens[3];
    assert_eq!("to", tokens[4]);
    let target   = tokens[5];
    (
        u8::from_str_radix(quantity, 10).expect(""),
        u8::from_str_radix(source, 10).expect(""),
        u8::from_str_radix(target, 10).expect(""),
    )
} 

#[test]
fn test_move_parsing() -> () {
    let (quantity, from, to) = parse_move_order(&String::from("move 4 from 3 to 2"));
    assert_eq!(quantity, 4);
    assert_eq!(from,     3);
    assert_eq!(to,       2);
}

fn move_items(stacks: &mut Vec<VecDeque<String>>, move_order: (u8, u8, u8), mutiple: bool) -> () {
    let source = move_order.1 as usize;
    let target = move_order.2 as usize;
    let quantity = move_order.0 as usize;
    if mutiple {
        let mut hold : Vec<String> = Vec::with_capacity(quantity);
        let src_stack = stacks.get_mut(source - 1)
            .expect("Source not found");
        for _ in 0..quantity {
            hold.push(src_stack.pop_front().expect("No item?"));
        }
        let dst_stack = stacks.get_mut(target - 1)
            .expect("Target not found");
        for val in hold.iter().rev() {
            dst_stack.push_front(val.clone());
        }
    }
    else { // One by one
        for _ in 0..quantity {
            let src_stack = stacks.get_mut(source - 1)
                .expect("Source not found");
            let taken = src_stack.pop_front().expect("No item?");
            let dst_stack = stacks.get_mut(target - 1)
                .expect("Target not found");
            dst_stack.push_front(taken.clone());
        }
    }
}

fn process(file_path: &dyn std::convert::AsRef<Path>, stack_count: usize, multiple: bool) -> Vec<String> {

    // File read
    let f = File::open(file_path).expect("Unable to open file");
    let mut reader = BufReader::new(f);
    let mut line_str = String::new();

    // Inventory
    let mut stacks = vec![VecDeque::new(); stack_count];

    // Reading
    let mut is_init : bool = true;
    loop {
        let len = reader.read_line(&mut line_str)
            .expect("Unable to read line");
        //println!("Line={} read={}", line_str, len);
        if len == 0 {
            break; // EOF
        }
        else if line_str.is_empty() || len == 1 /* \n */ {
            if is_init {
                println!("Start moves");
                is_init = false;
            }
            else {
                println!("quit");
                break;
            }
        }
        else if line_str.starts_with(" 1") {
            println!("Ignore line");
            // Instruction to be ignored
        }
        else {
            let input_line = line_str.clone();
            //rintln!("Before {:?}", stacks);
            if is_init {
                let stack_line : Vec<Option<String>> = parse_stack_info(&input_line, stack_count);
                process_stack_info(&mut stacks, &stack_line);
            } else {
                let (q, f, t) = parse_move_order(&input_line);
                //println!("Parsed move: {}x {}=>{}", q,f,t);
                move_items(&mut stacks, (q,f,t), multiple);
            }
            //println!("After  {:?}", stacks);
        }
        line_str.clear();
    }

    //println!("{:?}", stacks);
    stacks.iter()
        .map(|deq| deq.iter().next().expect("No item?")) // first element
        .map(|str| str.clone())
        .collect()
}


#[test]
fn test_example1() {
    let result = process(&"test.log", 3, false);
    assert_eq!("CMZ", result.join("").as_str());
}

#[test]
fn test_example2() {
    let result = process(&"test.log", 3, true);
    assert_eq!("MCD", result.join("").as_str());
}

fn main() {
    // 1st argument is the stack count
    let args_vec : Vec<String> = env::args().collect();
    let mut args = args_vec.iter();
    //println!("Arguments: {:?}", args);
    args.next(); // Program name
    let file_path = args.next().expect("File path is mandatory");
    println!("Reading file {}", file_path);
    let stack_count: usize = args.next()
        .map(|value| 
            usize::from_str_radix(value, 10)
                .expect("Impossible to parse stack count"))
        .unwrap_or(9);
    println!("Stack cound {}", stack_count);
    let multiple_mode : bool = args.next()
        .map(|arg| arg.parse().expect("Invalid boolean value"))
        .unwrap_or(true);
    println!("Multiple mode {}", multiple_mode);
    
    // Compute
    let result = process(file_path, stack_count, multiple_mode);

    // Print
    println!("{:?}", result.join(""));
}