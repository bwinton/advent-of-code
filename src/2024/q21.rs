//-----------------------------------------------------
// Setup.

use std::{collections::HashMap, sync::OnceLock};

use itertools::Itertools;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Move {
    North,
    East,
    South,
    West,
    Activate
}

static INPUT: &str = include_str!("data/q21.data");
static CELL: OnceLock<HashMap<(Option<usize>, Option<usize>), Vec<Move>>> = OnceLock::new();
static META_CELL: OnceLock<HashMap<(Move, Move), Vec<Move>>> = OnceLock::new(); 

fn parse(data: &str) -> Vec<(Vec<usize>, usize)> {
    let mut codes = vec![];
    for line in data.lines() {
        let mut numbers = vec![];
        let mut mult = 0;
        for c in line.chars() {
            if c.is_numeric() {
                let c = c.to_digit(10).unwrap() as usize;
                numbers.push(c);
                mult *= 10;
                mult += c;
                continue;
            }
            if c != 'A' {
                panic!("Unknown character {}!", c);
            }
        }
        codes.push((numbers, mult));
    }
    codes
}

fn get_moves() -> &'static HashMap<(Option<usize>, Option<usize>), Vec<Move>> {
    let moves = CELL.get_or_init(|| {
        HashMap::from([
            ((None, None), vec![]),
            ((None, Some(0)), vec![Move::West]),
            ((None, Some(1)), vec![Move::North, Move::West, Move::West]),
            ((None, Some(2)), vec![Move::West, Move::North]),
            ((None, Some(3)), vec![Move::North]),
            ((None, Some(4)), vec![Move::North, Move::North, Move::West, Move::West]),
            ((None, Some(5)), vec![Move::West, Move::North, Move::North]),
            ((None, Some(6)), vec![Move::North, Move::North]),
            ((None, Some(7)), vec![Move::North, Move::North, Move::North, Move::West, Move::West]),
            ((None, Some(8)), vec![Move::West, Move::North, Move::North, Move::North]),
            ((None, Some(9)), vec![Move::North, Move::North, Move::North]),

            ((Some(0), None), vec![Move::East]),
            ((Some(0), Some(0)), vec![]),
            ((Some(0), Some(1)), vec![Move::North, Move::West]),
            ((Some(0), Some(2)), vec![Move::North]),
            ((Some(0), Some(3)), vec![Move::North, Move::East]),
            ((Some(0), Some(4)), vec![Move::North, Move::North, Move::West]),
            ((Some(0), Some(5)), vec![Move::North, Move::North]),
            ((Some(0), Some(6)), vec![Move::North, Move::North, Move::East]),
            ((Some(0), Some(7)), vec![Move::North, Move::North, Move::North, Move::West]),
            ((Some(0), Some(8)), vec![Move::North, Move::North, Move::North]),
            ((Some(0), Some(9)), vec![Move::North, Move::North, Move::North, Move::East]),

            ((Some(1), None), vec![Move::East, Move::East, Move::South]),
            ((Some(1), Some(0)), vec![Move::East, Move::South]),
            ((Some(1), Some(1)), vec![]),
            ((Some(1), Some(2)), vec![Move::East]),
            ((Some(1), Some(3)), vec![Move::East, Move::East]),
            ((Some(1), Some(4)), vec![Move::North]),
            ((Some(1), Some(5)), vec![Move::North, Move::East]),
            ((Some(1), Some(6)), vec![Move::North, Move::East, Move::East]),
            ((Some(1), Some(7)), vec![Move::North, Move::North]),
            ((Some(1), Some(8)), vec![Move::North, Move::North, Move::East]),
            ((Some(1), Some(9)), vec![Move::North, Move::North, Move::East, Move::East]),

            ((Some(2), None), vec![Move::South, Move::East]),
            ((Some(2), Some(0)), vec![Move::South]),
            ((Some(2), Some(1)), vec![Move::West]),
            ((Some(2), Some(2)), vec![]),
            ((Some(2), Some(3)), vec![Move::East]),
            ((Some(2), Some(4)), vec![Move::West, Move::North]),
            ((Some(2), Some(5)), vec![Move::North]),
            ((Some(2), Some(6)), vec![Move::North, Move::East]),
            ((Some(2), Some(7)), vec![Move::West, Move::North, Move::North]),
            ((Some(2), Some(8)), vec![Move::North, Move::North]),
            ((Some(2), Some(9)), vec![Move::North, Move::North, Move::East]),

            ((Some(3), None), vec![Move::South]),
            ((Some(3), Some(0)), vec![Move::West, Move::South]),
            ((Some(3), Some(1)), vec![Move::West, Move::West]),
            ((Some(3), Some(2)), vec![Move::West]),
            ((Some(3), Some(3)), vec![]),
            ((Some(3), Some(4)), vec![Move::West, Move::West, Move::North]),
            ((Some(3), Some(5)), vec![Move::West, Move::North]),
            ((Some(3), Some(6)), vec![Move::North]),
            ((Some(3), Some(7)), vec![Move::West, Move::West, Move::North, Move::North]),
            ((Some(3), Some(8)), vec![Move::West, Move::North, Move::North]),
            ((Some(3), Some(9)), vec![Move::North, Move::North]),

            ((Some(4), None), vec![Move::East, Move::East, Move::South, Move::South]),
            ((Some(4), Some(0)), vec![Move::East, Move::South, Move::South]),
            ((Some(4), Some(1)), vec![Move::South]),
            ((Some(4), Some(2)), vec![Move::South, Move::East]),
            ((Some(4), Some(3)), vec![Move::South, Move::East, Move::East]),
            ((Some(4), Some(4)), vec![]),
            ((Some(4), Some(5)), vec![Move::East]),
            ((Some(4), Some(6)), vec![Move::East, Move::East]),
            ((Some(4), Some(7)), vec![Move::North]),
            ((Some(4), Some(8)), vec![Move::North, Move::East]),
            ((Some(4), Some(9)), vec![Move::North, Move::East, Move::East]),

            ((Some(5), None), vec![Move::South, Move::South, Move::East]),
            ((Some(5), Some(0)), vec![Move::South, Move::South]),
            ((Some(5), Some(1)), vec![Move::West, Move::South]),
            ((Some(5), Some(2)), vec![Move::South]),
            ((Some(5), Some(3)), vec![Move::South, Move::East]),
            ((Some(5), Some(4)), vec![Move::West]),
            ((Some(5), Some(5)), vec![]),
            ((Some(5), Some(6)), vec![Move::East]),
            ((Some(5), Some(7)), vec![Move::West, Move::North]),
            ((Some(5), Some(8)), vec![Move::North]),
            ((Some(5), Some(9)), vec![Move::North, Move::East]),

            ((Some(6), None), vec![Move::South, Move::South]),
            ((Some(6), Some(0)), vec![Move::West, Move::South, Move::South]),
            ((Some(6), Some(1)), vec![Move::West, Move::West, Move::South]),
            ((Some(6), Some(2)), vec![Move::West, Move::South]),
            ((Some(6), Some(3)), vec![Move::South]),
            ((Some(6), Some(4)), vec![Move::West, Move::West]),
            ((Some(6), Some(5)), vec![Move::West]),
            ((Some(6), Some(6)), vec![]),
            ((Some(6), Some(7)), vec![Move::West, Move::West, Move::North]),
            ((Some(6), Some(8)), vec![Move::West, Move::North]),
            ((Some(6), Some(9)), vec![Move::North]),

            ((Some(7), None), vec![Move::East, Move::East, Move::South, Move::South, Move::South]),
            ((Some(7), Some(0)), vec![Move::East, Move::South, Move::South, Move::South]),
            ((Some(7), Some(1)), vec![Move::South, Move::South]),
            ((Some(7), Some(2)), vec![Move::South, Move::South, Move::East]),
            ((Some(7), Some(3)), vec![Move::South, Move::South, Move::East, Move::East]),
            ((Some(7), Some(4)), vec![Move::South]),
            ((Some(7), Some(5)), vec![Move::South, Move::East]),
            ((Some(7), Some(6)), vec![Move::South, Move::East, Move::East]),
            ((Some(7), Some(7)), vec![]),
            ((Some(7), Some(8)), vec![Move::East]),
            ((Some(7), Some(9)), vec![Move::East, Move::East]),

            ((Some(8), None), vec![Move::South, Move::South, Move::South, Move::East]),
            ((Some(8), Some(0)), vec![Move::South, Move::South, Move::South]),
            ((Some(8), Some(1)), vec![Move::West, Move::South, Move::South]),
            ((Some(8), Some(2)), vec![Move::South, Move::South]),
            ((Some(8), Some(3)), vec![Move::South, Move::South, Move::East]),
            ((Some(8), Some(4)), vec![Move::West, Move::South]),
            ((Some(8), Some(5)), vec![Move::South]),
            ((Some(8), Some(6)), vec![Move::South, Move::East]),
            ((Some(8), Some(7)), vec![Move::West]),
            ((Some(8), Some(8)), vec![]),
            ((Some(8), Some(9)), vec![Move::East]),

            ((Some(9), None), vec![Move::South, Move::South, Move::South]),
            ((Some(9), Some(0)), vec![Move::West, Move::South, Move::South, Move::South]),
            ((Some(9), Some(1)), vec![Move::West, Move::West, Move::South, Move::South]),
            ((Some(9), Some(2)), vec![Move::West, Move::South, Move::South]),
            ((Some(9), Some(3)), vec![Move::South, Move::South]),
            ((Some(9), Some(4)), vec![Move::West, Move::West, Move::South]),
            ((Some(9), Some(5)), vec![Move::West, Move::South]),
            ((Some(9), Some(6)), vec![Move::South]),
            ((Some(9), Some(7)), vec![Move::West, Move::West]),
            ((Some(9), Some(8)), vec![Move::West]),
            ((Some(9), Some(9)), vec![]),
        ])
    });
    moves
}

fn get_meta_moves() -> &'static HashMap<(Move, Move), Vec<Move>> {
    let meta_moves = META_CELL.get_or_init(|| {
        HashMap::from([
            ((Move::Activate, Move::Activate), vec![]),
            ((Move::Activate, Move::North), vec![Move::West]),
            ((Move::Activate, Move::East), vec![Move::South]),
            ((Move::Activate, Move::South), vec![Move::West, Move::South]),
            ((Move::Activate, Move::West), vec![Move::South, Move::West, Move::West]),

            ((Move::North, Move::Activate), vec![Move::East]),
            ((Move::North, Move::North), vec![]),
            ((Move::North, Move::East), vec![Move::South, Move::East]),
            ((Move::North, Move::South), vec![Move::South]),
            ((Move::North, Move::West), vec![Move::South, Move::West]),

            ((Move::East, Move::Activate), vec![Move::North]),
            ((Move::East, Move::North), vec![Move::West, Move::North]),
            ((Move::East, Move::East), vec![]),
            ((Move::East, Move::South), vec![Move::West]),
            ((Move::East, Move::West), vec![Move::West, Move::West]),

            ((Move::South, Move::Activate), vec![Move::North, Move::East]),
            ((Move::South, Move::North), vec![Move::North]),
            ((Move::South, Move::East), vec![Move::East]),
            ((Move::South, Move::South), vec![]),
            ((Move::South, Move::West), vec![Move::West]),

            ((Move::West, Move::Activate), vec![Move::East, Move::East, Move::North]),
            ((Move::West, Move::North), vec![Move::East, Move::North]),
            ((Move::West, Move::East), vec![Move::East, Move::East]),
            ((Move::West, Move::South), vec![Move::East]),
            ((Move::West, Move::West), vec![]),

            ])
        });
    meta_moves
}

fn get_path(code: Vec<usize>) -> Vec<Move> {
    let moves = get_moves();

    let mut rv= vec![];
    // println!("{:?}", code);
    let mut curr = None;
    for number in code {
        let path = &moves[&(curr, Some(number))];
        // println!("{:?} -> {:?}: {:?}", curr, Some(number), path);
        rv.extend(path);
        rv.push(Move::Activate);
        curr = Some(number);
    }
    let path = &moves[&(curr, None)];
    // println!("{:?} -> None: {:?}", curr, path);
    rv.extend(path);
    rv.push(Move::Activate);
    // println!("rv: {:?}", rv);
    rv
}


fn get_meta_path(start: Move, moves: Vec<Move>) -> Vec<Move> {
    let meta_moves = get_meta_moves();
    
    let mut rv= vec![];
    // println!("  moves: {:?}", moves);
    let mut curr = start;
    for next in moves {
        let path = &meta_moves[&(curr, next)];
        // println!("  move: {:?} -> {:?}: {:?}", curr, next, path);
        rv.extend(path);
        rv.push(Move::Activate);
        curr = next;
    }
    // println!("  rv: {:?}", rv);
    rv    
}

fn get_meta_path_len(prev: Move, curr: Move, level: usize, cache: &mut HashMap<(Move, Move, usize), usize>) -> usize {
        let mut rv= 0;

        println!("{}{:?} -> {:?}", "  ".repeat(2-level), prev, curr);

        if level == 0 {
            let meta_moves = get_meta_moves();
            println!("{}:Base case {}: {:?}!", "  ".repeat(2-level), meta_moves[&(prev, curr)].len() + 1, meta_moves[&(prev, curr)]);
            return meta_moves[&(prev, curr)].len() + 1;
        }

        rv += if let Some(&len) = cache.get(&(prev, curr, level)) {
            println!("{}:Found {}!", "  ".repeat(2-level), len);
            len
        } else {
            let meta_path = get_meta_path(vec![prev, prev, curr]);
            println!("{}:Calculating {:?}!", "  ".repeat(2-level), meta_path);
            let mut len = 0;
            for (prev, curr) in meta_path.into_iter().tuple_windows() {
                let curr_len = get_meta_path_len(prev, curr, level - 1, cache);
                len += curr_len;
            }
            cache.insert((prev, curr, level), len);
            println!("{}:Calculated {}!", "  ".repeat(2-level), len);
            len
        };
        // println!("  move: {:?} -> {:?}: {:?}", prev, curr, path);
        // println!("  rv: {:?}", rv);
        rv    
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let codes = parse(data);
    let mut cache = HashMap::new();
    for (code, mult) in codes {
        println!("code: {}{}{}", code[0], code[1], code[2]);
        let path = get_path(code);
        println!("path: {:?}", path);
        let mut len = 0;
        for (prev, curr) in path.into_iter().tuple_windows() {
            let curr_len = get_meta_path_len(prev, curr, 1, &mut cache);
            len += curr_len;
        }
        println!("  len: {}", len);
        rv += len * mult;
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let codes = parse(data);
    let mut cache = HashMap::new();
    for (code, mult) in codes {
        println!("code: {}{}{}", code[0], code[1], code[2]);
        let path = get_path(code);
        let mut len = 0;
        for (prev, curr) in path.into_iter().tuple_windows() {
            let curr_len = get_meta_path_len(prev, curr, 2, &mut cache);
            len += curr_len;
        }
        println!("  len: {}", len);
        rv += len * mult;
    }
    rv
    // 3,231,481,965,789,453,889 is too high.
    // 9,684,970,575,674,898,655
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("029A"), 68 * 29);
//     assert_eq!(process_data_a("980A"), 60 * 980);
//     assert_eq!(process_data_a("179A"), 68 * 179);
//     assert_eq!(process_data_a("456A"), 64 * 456);
//     assert_eq!(process_data_a("379A"), 64 * 379);

//     assert_eq!(process_data_a(indoc!("
//         029A
//         980A
//         179A
//         456A
//         379A")), 126_384);
}

#[test]
fn b() {
    // use pretty_assertions::assert_eq;

    // assert_eq!(process_data_b(indoc!("")), 0);
}
