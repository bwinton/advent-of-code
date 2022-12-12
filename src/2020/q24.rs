use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;
//-----------------------------------------------------
// Setup.
use regex::Regex;

static INPUT: &str = include_str!("data/q24.data");

static DIR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(e|se|sw|w|nw|ne)").unwrap());

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::East,
    Direction::SouthEast,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
    Direction::NorthEast,
];

impl Direction {
    fn from_str(dir: &str) -> Self {
        match dir {
            "e" => Direction::East,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            "w" => Direction::West,
            "nw" => Direction::NorthWest,
            "ne" => Direction::NorthEast,
            _ => panic!("Unknown direction: {}", dir),
        }
    }

    fn to_coords(&self) -> (isize, isize) {
        match self {
            Direction::East => (1, 0),
            Direction::SouthEast => (1, -1),
            Direction::SouthWest => (0, -1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, 1),
            Direction::NorthEast => (0, 1),
        }
    }
}

fn process_data_a(data: &str) -> usize {
    let mut tiles = vec![];
    for line in data.lines() {
        let path: Vec<_> = DIR_RE
            .find_iter(line)
            .map(|dir| Direction::from_str(dir.as_str()))
            .collect();
        tiles.push(path);
    }
    let mut board = HashSet::new();
    for path in tiles {
        let mut curr = (0, 0);
        for dir in path {
            let next = dir.to_coords();
            curr.0 += next.0;
            curr.1 += next.1;
        }
        if board.contains(&curr) {
            board.remove(&curr);
        } else {
            board.insert(curr);
        }
    }
    board.len()
}

fn run(board: HashSet<(isize, isize)>) -> HashSet<(isize, isize)> {
    // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    let mut nearby = HashMap::new();
    for &tile in &board {
        for direction in ALL_DIRECTIONS.iter() {
            let next = direction.to_coords();
            let neighbour = (tile.0 + next.0, tile.1 + next.1);
            *nearby.entry(neighbour).or_insert(0) += 1;
        }
    }
    let mut rv = HashSet::new();
    for (tile, count) in nearby {
        if (board.contains(&tile) && (count == 1 || count == 2))
            || (!board.contains(&tile) && count == 2)
        {
            rv.insert(tile);
        }
    }

    rv
}

fn process_data_b(data: &str) -> usize {
    let mut tiles = vec![];
    for line in data.lines() {
        let path: Vec<_> = DIR_RE
            .find_iter(line)
            .map(|dir| Direction::from_str(dir.as_str()))
            .collect();
        tiles.push(path);
    }
    let mut board = HashSet::new();
    for path in tiles {
        let mut curr = (0, 0);
        for dir in path {
            let next = dir.to_coords();
            curr.0 += next.0;
            curr.1 += next.1;
        }
        if board.contains(&curr) {
            board.remove(&curr);
        } else {
            board.insert(curr);
        }
    }

    // got the initial board. Now run 100 days.
    for _ in 1..=100 {
        board = run(board);
        // println!("{}: {}", i, board.len());
    }
    board.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("24");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "sesenwnenenewseeswwswswwnenewsewsw
            neeenesenwnwwswnenewnwwsewnenwseswesw
            seswneswswsenwwnwse
            nwnwneseeswswnenewneswwnewseswneseene
            swweswneswnenwsewnwneneseenw
            eesenwseswswnenwswnwnwsewwnwsene
            sewnenenenesenwsewnenwwwse
            wenwwweseeeweswwwnwwe
            wsweesenenewnwwnwsenewsenwwsesesenwne
            neeswseenwwswnwswswnw
            nenwswwsewswnenenewsenwsenwnesesenew
            enewnwewneswsewnwswenweswnenwsenwsw
            sweneswneswneneenwnewenewwneswswnese
            swwesenesewenwneswnwwneseswwne
            enesenwswwswneneswsenwnewswseenwsese
            wnwnesenesenenwwnenwsewesewsesesew
            nenewswnwewswnenesenwnesewesw
            eneswnwswnwsenenwnwnwwseeswneewsenese
            neswnwewnwnwseenwseesewsenwsweewe
            wseweeenwnesenwwwswnew"
        )),
        10
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "sesenwnenenewseeswwswswwnenewsewsw
            neeenesenwnwwswnenewnwwsewnenwseswesw
            seswneswswsenwwnwse
            nwnwneseeswswnenewneswwnewseswneseene
            swweswneswnenwsewnwneneseenw
            eesenwseswswnenwswnwnwsewwnwsene
            sewnenenenesenwsewnenwwwse
            wenwwweseeeweswwwnwwe
            wsweesenenewnwwnwsenewsenwwsesesenwne
            neeswseenwwswnwswswnw
            nenwswwsewswnenenewsenwsenwnesesenew
            enewnwewneswsewnwswenweswnenwsenwsw
            sweneswneswneneenwnewenewwneswswnese
            swwesenesewenwneswnwwneseswwne
            enesenwswwswneneswsenwnewswseenwsese
            wnwnesenesenenwwnenwsewesewsesesew
            nenewswnwewswnenesenwnesewesw
            eneswnwswnwsenenwnwnwwseeswneewsenese
            neswnwewnwnwseenwseesewsenwsweewe
            wseweeenwnesenwwwswnew"
        )),
        2208
    );
}
