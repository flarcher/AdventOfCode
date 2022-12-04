use std::env;
use std::io;

static DEFAULT_FILTER : usize = 1;

fn main() {

    // Arguments
    let args: Vec<String> = env::args().collect();
    let filter_count : usize;
    if &args.len() > &1 {
      let tmp = &args.get(1)
        .map(|s| s.parse::<usize>())
        .or(Option::Some(Result::Ok(DEFAULT_FILTER)))
        .unwrap()
        .expect("Internal error");
      filter_count = *tmp;
    }
    else {
      filter_count = DEFAULT_FILTER;
    }

    // Parsing
    let mut counters : Vec<u32> = Vec::new();
    let mut line = String::new();
    let stdin = io::stdin();
    let mut count : u32 = 0;
    loop {
        let bytes_count = stdin.read_line(&mut line).unwrap();
        //println!("{}", line);
        let value_str = line.trim_end();
        if bytes_count <= 0 {
            break;
        }
        else if value_str.is_empty() {
            // New Aelve
            counters.push(count);
            count = 0;
        }
        else {
            //println!("{}", line);
            let line_value : u32 = value_str.parse().expect("Failed to parse line");
            count += line_value
        }
        line.clear();
    }

    // Output
    counters.sort_by(|l, r| l.cmp(r).reverse());
    let mut total : u32 = 0;
    for count in &counters.as_slice()[0..filter_count] {
        total += count;
        println!("{}", count);
    }

    println!("Total is {}", total);

}
