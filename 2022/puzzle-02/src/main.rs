
use std::env;
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Shape {
    Rock,
    Paper,
    Scissor
}

fn play(player : &Shape, opponent : &Shape, may_recurse : bool) -> Ordering {
    if *player == *opponent {
        Ordering::Equal
    }
    else if
        (*player == Shape::Rock && *opponent == Shape::Scissor) ||
        (*player == Shape::Scissor && *opponent == Shape::Paper) ||
        (*player == Shape::Paper && *opponent == Shape::Rock) {
        Ordering::Greater
    }
    else if may_recurse {
        play(opponent, player, false).reverse()
    }
    else {
        panic!("Invalid state {:?} {:?}", player, opponent);
    }
}

fn get_score(
        score_map: &HashMap<Shape, i32>,
        player : &Shape,
        play_result : Ordering) -> i32 {
    let shape_score = *(score_map.get(player).expect("Unknown shape"));
    let play_score = match play_result {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6
        };
    shape_score + play_score
}

fn solve_shape(opponent : &Shape, result: Ordering) -> &Shape {
    match result {
        Ordering::Equal => opponent,
        Ordering::Less => {
            if *opponent == Shape::Rock {
                &Shape::Scissor
            } else if *opponent == Shape::Paper {
                &Shape::Rock
            } else { // SCISSOR
                &Shape::Paper
            }
        },
        Ordering::Greater => {
            if *opponent == Shape::Rock {
                &Shape::Paper
            } else if *opponent == Shape::Paper {
                &Shape::Scissor
            } else { // SCISSOR
                &Shape::Rock
            }
        }
    }
}

fn parse_line<'a>(mapper: &'a HashMap<u8, Shape>, line: &String) -> (&'a Shape, &'a Shape) {
    assert!(line.len() >= 3);
    let bytes = line.as_bytes();
    let opponent = bytes[0];
    assert!(opponent.is_ascii());
    let opponent_shape = mapper.get(&opponent).unwrap();
    let player = bytes[2];
    assert!(player.is_ascii());
    let player_shape = mapper.get(&player).unwrap();
    (opponent_shape, player_shape)
}

enum ReadingMode {
    Shape,
    PlayResult
}

fn get_score_from_line(
    char_map: &HashMap<u8, Shape>,
    score_map: &HashMap<Shape, i32>,
    result_map: &HashMap<Shape, Ordering>,
    line: &String,
    mode: &ReadingMode) -> i32 {

    let (opponent, player) = parse_line(char_map, line);

    let play_result : Ordering = match mode {
        ReadingMode::Shape => {
            play(&player, &opponent, true)
        },
        ReadingMode::PlayResult => {
            *(result_map.get(&player).expect("Result not found"))
        }
    };

    let player_shape = match mode {
        ReadingMode::Shape => {
            player
        },
        ReadingMode::PlayResult => {
            &(solve_shape(&opponent, play_result))
        }
    };

    get_score(score_map, player_shape, play_result)
}

fn main() {

    // Argument
    let args: Vec<String> = env::args().collect();
    let reading_mode : ReadingMode = if args.len() > 1
        && ! String::from("shape").eq_ignore_ascii_case(args.get(1).unwrap())
        { ReadingMode::PlayResult } else { ReadingMode::Shape };

    // Init
    let shape_scores = HashMap::from([
        (Shape::Rock,    1),
        (Shape::Paper,   2),
        (Shape::Scissor, 3),
    ]);
    let result_mapping = HashMap::from([
        (Shape::Rock, Ordering::Less),
        (Shape::Paper, Ordering::Equal),
        (Shape::Scissor, Ordering::Greater),
    ]);
    let parsing_mapping = HashMap::from([
        ("A".as_bytes()[0], Shape::Rock),
        ("B".as_bytes()[0], Shape::Paper),
        ("C".as_bytes()[0], Shape::Scissor),
        ("X".as_bytes()[0], Shape::Rock),
        ("Y".as_bytes()[0], Shape::Paper),
        ("Z".as_bytes()[0], Shape::Scissor),
    ]);

    // Reading
    let stdin : io::Stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut total : i32 = 0;
    loop {
        if let Some(line) = lines.next() {
          let str = line.expect("Impossible to read line");
          let line_value : i32 = get_score_from_line(
            &parsing_mapping, &shape_scores, &result_mapping,
            &str, &reading_mode);
          total += line_value;
        } else {
          break;
        }
    }

    // Printing
    println!("Total = {}", total);
}
