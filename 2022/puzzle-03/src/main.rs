use std::{io::{self, BufRead}, collections::HashSet};

use intersection;
//mod intersection;

fn to_set(s : &str) -> HashSet<u8> {
    s.as_bytes().iter().map(|c| *c).collect::<HashSet<u8>>()
}

fn get_redundant_item(backpack: &String) -> u8 {
    let separator = backpack.len() / 2;
    let left_pack = &backpack[..separator];
    assert_eq!(left_pack.len(), separator);
    let right_pack = &backpack[separator..];
    assert_eq!(right_pack.len(), separator);
    let left_set = to_set(left_pack);
    let right_set = to_set(right_pack);
    let duplicate = left_set.intersection(&right_set)
        .last()
        .expect("No redundant item type in backpack");
    *duplicate
}

fn get_char_byte(s : &str) -> u8 {
    s.as_bytes()[0]
}

static SCORE_A_LOWERCASE : u8 = "a".as_bytes()[0];
static SCORE_A_UPPERCASE : u8 = "A".as_bytes()[0];

fn get_item_score(c: u8) -> u8 {
    if c.is_ascii_lowercase() {
        c - SCORE_A_LOWERCASE + 1
    } else if c.is_ascii_uppercase() {
        c - SCORE_A_UPPERCASE + 27
    } else {
        panic!("Invalid character {:?}", c);
    }
}

fn get_badge_score(packs : &Vec<String>) -> i32 {
    let sets : Vec<HashSet<u8>> = packs.iter()
        .map(|s| to_set((*s).as_str()))
        .collect();
    let badge_set = intersection::hash_set::intersection(sets);
    assert_eq!(1, badge_set.len());
    let badge_char = *(badge_set.iter().last().expect("No badge item"));
    get_item_score(badge_char) as i32
}

fn main() {

    let mut redundancy_total : i32 = 0;
    let mut badges_total : i32 = 0;

    // Checking
    assert_eq!(1, get_item_score(SCORE_A_LOWERCASE));
    assert_eq!(26, get_item_score(get_char_byte("z")));
    assert_eq!(27, get_item_score(SCORE_A_UPPERCASE));
    assert_eq!(52, get_item_score(get_char_byte("Z")));

    // Reading
    let stdin : io::Stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut group_counter = 0;
    let mut group_packs : Vec<String> = Vec::with_capacity(3);
    loop {
        if let Some(line) = lines.next() {
            let line_str = line.expect("Unable to read line");

            let item_type = get_redundant_item(&line_str);
            let score = get_item_score(item_type);
            redundancy_total += score as i32;

            group_packs.push(String::from(line_str.as_str()));
            group_counter += 1;
            if group_counter == 3 {
                badges_total += get_badge_score(&group_packs);
                group_packs.clear();
                group_counter = 0;
            }

        } else {
            break;
        }
    }

    println!("Redondant items total is {}", redundancy_total);
    println!("Badges items total is {}", badges_total);

}
