use std::cmp::Ordering;
use std::io;
use std::collections::HashMap;

static ROCK_OTHER     : u8 = "A".as_bytes()[0];
static PAPER_OTHER    : u8 = "B".as_bytes()[0];
static SCISSOR_OTHER  : u8 = "C".as_bytes()[0];

static ROCK_PLAYER    : u8 = "X".as_bytes()[0];
static PAPER_PLAYER   : u8 = "Y".as_bytes()[0];
static SCISSOR_PLAYER : u8 = "Z".as_bytes()[0];

fn get_shape_score(score_map: & HashMap<u8, i32>, shape : u8) -> i32 {
    if ! shape.is_ascii() {
        panic!("Shape {} is not valid", shape);
    }
    else {
        match score_map.get(&shape) {
            Some(v) => *v,
            None => panic!("Unknown shape {}", shape)
        }
    }
}

fn play(player : u8, opponent : u8) -> Ordering {
    if player == opponent {
        Ordering::Equal
    }
    else if
        (player == ROCK_PLAYER && opponent == SCISSOR_PLAYER) ||
        (player == SCISSOR_PLAYER && opponent == PAPER_PLAYER) ||
        (player == PAPER_PLAYER && opponent == ROCK_PLAYER) {
        Ordering::Greater
    }
    else {
        play(opponent, player).reverse()
    }
}

fn get_score(
        score_map: &HashMap<u8, i32>,
        player : u8,
        play_result : Ordering) -> i32 {
    let shape_score = get_shape_score(score_map, player);
    let play_score = match play_result {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6
        };
    shape_score + play_score
}

fn solve_shape(opponent : u8, result: Ordering) -> u8 {
    match result {
        Ordering::Equal => opponent,
        Ordering::Less => {
            if opponent == ROCK_PLAYER {
                SCISSOR_PLAYER
            } else if opponent == PAPER_PLAYER {
                ROCK_PLAYER
            } else { // SCISSOR
                PAPER_PLAYER
            }
        },
        Ordering::Greater => {
            if opponent == ROCK_PLAYER {
                PAPER_PLAYER
            } else if opponent == PAPER_PLAYER {
                SCISSOR_PLAYER
            } else { // SCISSOR
                ROCK_PLAYER
            }
        }
    }
}

fn parse_line(mapper: &HashMap<u8, u8>, line: &String) -> (u8, u8) {
    assert!(line.len() >= 3);
    let bytes = line.as_bytes();
    let opponent = mapper.get(&bytes[0]).unwrap();
    let player = bytes[2];
    (*opponent, player)
}

enum ReadingMode {
    Shape,
    PlayResult
}

fn get_score_from_line(
    char_map: &HashMap<u8, u8>,
    score_map: &HashMap<u8, i32>,
    result_map: &HashMap<u8, Ordering>,
    line: &String,
    mode: &ReadingMode) -> i32 {
    
    let (opponent, player) = parse_line(char_map, line);
    
    let play_result : Ordering = match mode {
        ReadingMode::Shape => {
            play(player, opponent)
        },
        ReadingMode::PlayResult => {
            *(result_map.get(&player).unwrap())
        }
    };

    let player_shape = match mode {
        ReadingMode::Shape => {
            player
        },
        ReadingMode::PlayResult => {
            solve_shape(opponent, play_result)
        }
    };

    get_score(score_map, player_shape, play_result)
}

fn main() {

    // Argument
    let reading_mode : ReadingMode = ReadingMode::PlayResult; 

    // Init
    assert!(ROCK_PLAYER != PAPER_PLAYER);
    assert!(PAPER_PLAYER != SCISSOR_PLAYER);
    assert!(SCISSOR_PLAYER != ROCK_PLAYER);
    let shape_scores = HashMap::from([
        (ROCK_PLAYER,    1),
        (PAPER_PLAYER,   2),
        (SCISSOR_PLAYER, 3)
    ]);
    let shape_mapping = HashMap::from([
        (ROCK_OTHER, ROCK_PLAYER),
        (PAPER_OTHER, PAPER_PLAYER),
        (SCISSOR_OTHER, SCISSOR_PLAYER)
    ]);
    let result_mapping = HashMap::from([
        (ROCK_PLAYER, Ordering::Less),
        (PAPER_PLAYER, Ordering::Equal),
        (SCISSOR_PLAYER, Ordering::Greater)
    ]);
    let stdin : io::Stdin = io::stdin();

    // Reading
    let mut line = String::new();
    let mut total : i32 = 0;
    loop {
        let bytes_count = stdin.read_line(&mut line).unwrap();
        if bytes_count <= 0 {
            break;
        }
        else {
            let line_value : i32 = get_score_from_line(
                &shape_mapping, &shape_scores, &result_mapping,
                &line, &reading_mode);
            total += line_value;
        }
        line.clear();
    }

    // Printing
    println!("Total = {}", total);
}
