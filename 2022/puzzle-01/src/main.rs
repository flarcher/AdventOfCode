use std::env;
use std::io;

static DEFAULT_FILTER : usize = 1;

fn get_filter_count() -> usize {
  let args: Vec<String> = env::args().collect();
  if &args.len() > &1 {
    let tmp = &args.get(1)
      .map(|s| s.parse::<usize>())
      .or(Option::Some(Result::Ok(DEFAULT_FILTER)))
      .unwrap()
      .expect("Internal error");
    *tmp
  }
  else {
    DEFAULT_FILTER
  }
}

fn sort_aelves(stdin : io::Stdin, counters : &mut Vec<u32>) -> () {
  let mut line = String::new();
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

  counters.sort_by(|l, r| l.cmp(r).reverse());
}

fn print_aelves(slice : &[u32], filter_count : usize) -> () {
  let mut total : u32 = 0;
  for count in &slice[0..filter_count] {
      total += count;
      println!("{}", count);
  }
  println!("Total is {}", total);
}

fn main() {
    let stdin        : io::Stdin = io::stdin();
    let filter_count : usize     = get_filter_count();
    let mut counters : Vec<u32>  = Vec::new();
    sort_aelves(stdin, &mut counters);
    print_aelves(&counters.as_slice(), filter_count);
}
