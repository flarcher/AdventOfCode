use std::io::{self, BufRead};
use std::{env, usize};
use std::collections::VecDeque;


fn parse_stack_symbol(symbol: &str) -> Option<u8> {
    let val = symbol.as_bytes()[0];
    if val.is_ascii_uppercase() { Some(val) } else { None }
}

fn parse_stack_info(line : &String, count: usize) -> Vec<Option<u8>> {
    let mut result: Vec<Option<u8>> = Vec::with_capacity(count);
    for stack in 0..count {
        let part_index = stack * 4;
        let opt_value = parse_stack_symbol(&line[part_index+1..part_index+2]);
        result[stack] = opt_value;
    }
    result
}

fn process_stack_info(stacks: &mut Vec<VecDeque<u8>>, input: &Vec<Option<u8>>) -> () {
    for (index, stack) in input.iter().enumerate() {
        match stack {
            None => (),
            Some(item) => stacks[index].push_back(*item)
        }
    }
}

// move q from f to t
fn parse_move_order(line: &String) -> (u8, u8, u8) {
    assert_eq!(18, line.len());
    let quantity = line[5..6].as_bytes()[0];
    let source   = line[12..13].as_bytes()[0];
    let target   = line[17..18].as_bytes()[0];
    (quantity, source, target)
} 

fn move_items(stacks: &mut Vec<VecDeque<u8>>, move_order: (u8, u8, u8)) -> () {
    let source = move_order.1 as usize;
    let target = move_order.2 as usize;
    let quantity = move_order.0 as usize;
    let mut moved : Vec<u8> = Vec::with_capacity(quantity);
    let src_stack = stacks.get_mut(source)
        .expect("Source not found");
    for _ in 0..quantity {
        let taken = src_stack.pop_front();
        moved.push(taken.expect("Not enough item"));
    }
    let dst_stack = stacks.get_mut(target)
            
        .expect("Target not found");
    for item in moved.iter().rev() {
        dst_stack.push_front(*item);
    }
}

fn main() {

    // 1st argument is the stack count
    let args: Vec<String> = env::args().collect();
    let stack_count: usize = if args.len() > 1 {
            usize::from_str_radix(
                args.get(1).unwrap(), 
                10)
                .expect("Impossible to parse stack count")
        } else {
            9 // Default
        };

    // Inventory
    let mut stacks: Vec<VecDeque<u8>> = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(VecDeque::new()); // push_back
    }
    assert_eq!(stack_count, stacks.len());

    // Reading
    let mut is_init : bool = true;
    let stdin : io::Stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        if let Some(line) = lines.next() {
            let line_str = line.expect("Unable to read line");
            if line_str.len() >= 1 && line_str[0..1].eq(" ") {
                continue; // Instruction to be ignored
            }
            else if line_str.is_empty() {
                is_init = false;
                continue; // Separator
            }
            else if is_init {
                let stack_line : Vec<Option<u8>> = parse_stack_info(&line_str, stack_count);
                process_stack_info(&mut stacks, &stack_line);
            } else {
                let (q, f, t) = parse_move_order(&line_str);
                move_items(&mut stacks, (q,f,t));
            }
        }
        else {
            break;
        }
    }

    println!("Hello, world!");
}
