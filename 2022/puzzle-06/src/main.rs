use std::{env, fs};


/// Returns `true` only if all characters are different.
fn different_chars(input: &str) -> bool {
    let chars = input.as_bytes();
    for i in 0..(chars.len()-1) {
        for j in (i+1)..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}

/**
  Returns the first index followed by n different characters.
  
  Or `0` if not found. 
*/
fn get_start_index(input: &str, size: usize) -> usize {
    assert!(size > 1);
    let mut first_chars : &str = &input[0..size];
    let max_index = input.len()-1;
    for i in size..max_index {
        if different_chars(first_chars) {
            return i;
        }
        else {
            let end_index = i + 1;
            let start_index = end_index - size;
            first_chars = &input[start_index..end_index];
        }
    }
    0 // Not found
}

fn main() {
    let args_collection = env::args().collect::<Vec<String>>();
    let mut args = args_collection.iter();
    assert!(args.next().is_some());
    let file_name = args.next().expect("No file name given");
    let size : usize = args.next()
        .map_or(4, |arg| arg.parse::<usize>().expect("Impossible to parse argument"));
    println!("Condition: {} different characters", size);

    let file_content = fs::read_to_string(file_name).expect("Impossible to read file");

    let index = get_start_index(&file_content, size);
    println!("Index is {}", index);
}

#[test]
fn test_1() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(7, get_start_index(input, 4));
}

#[test]
fn test_2() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(5, get_start_index(input, 4));
}

#[test]
fn test_3() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(6, get_start_index(input, 4));
}

#[test]
fn test_4() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(10, get_start_index(input, 4));
}

#[test]
fn test_5() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(11, get_start_index(input, 4));
}

#[test]
fn test_6() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(19, get_start_index(input, 14));
}

#[test]
fn test_7() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(23, get_start_index(input, 14));
}

#[test]
fn test_8() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(23, get_start_index(input, 14));
}

#[test]
fn test_9() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(29, get_start_index(input, 14));
}

#[test]
fn test_10() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(26, get_start_index(input, 14));
}