//-----------------------------------------------------
// Setup.

use std::{collections::HashMap, iter, ops::RangeInclusive};

use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

static INPUT: &str = include_str!("data/q22.data");

type Face = (RangeInclusive<i64>, RangeInclusive<i64>);

#[derive(Debug)]
enum Move {
    Forward(usize),
    Left,
    Right,
}
impl Move {
    fn update(
        &self,
        curr: (usize, usize, Direction),
        faces: &[Face],
        map: &[Vec<Cell>],
        move_map: &HashMap<(usize, Direction), (usize, Direction)>,
    ) -> (usize, usize, Direction) {
        let (mut x, mut y, mut direction) = curr;
        match self {
            Move::Forward(value) => {
                // println!("Moving {:?} from {:?}", self, (x, y, &direction));
                let (mut next_x, mut next_y) = (x, y);

                // println!(" to {:?}", curr);
                for _i in 0..*value {
                    // Move one step in the direction, wrapping around, stopping at walls.
                    let curr = (next_x, next_y);
                    let face = get_face(curr.0, curr.1, faces).unwrap();
                    (next_x, next_y, direction) = direction.get_next(curr, face, faces, move_map);
                    match map[next_y][next_x] {
                        Cell::Empty => {
                            // println!(
                            //     "  {}: Empty. Continuing to {:?}.",
                            //     _i,
                            //     (next_x, next_y, &direction)
                            // );
                        }
                        Cell::Wall => {
                            // Can't move.
                            // println!(
                            //     "  {}: Hit a wall. Stopping before {:?}.",
                            //     _i,
                            //     (next_x, next_y, &direction)
                            // );
                            (next_x, next_y) = curr;
                            break;
                        }
                        Cell::OutOfBounds => panic!("Shouldn't have found this cell!"),
                    }
                }
                (x, y) = (next_x, next_y);
            }
            Move::Left => {
                direction = match direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                }
            }
            Move::Right => {
                direction = match direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                }
            }
        }
        (x, y, direction)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    OutOfBounds,
    Empty,
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn get_value(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn get_next(
        &self,
        curr: (usize, usize),
        face: usize,
        faces: &[Face],
        move_map: &HashMap<(usize, Direction), (usize, Direction)>,
    ) -> (usize, usize, Direction) {
        let temp = match self {
            Direction::Right => (curr.0 as i64 + 1, curr.1 as i64),
            Direction::Down => (curr.0 as i64, curr.1 as i64 + 1),
            Direction::Left => (curr.0 as i64 - 1, curr.1 as i64),
            Direction::Up => (curr.0 as i64, curr.1 as i64 - 1),
        };

        let curr_face = faces[face].clone();
        print!("  Testing {:?} in {:?} ({})?", temp, curr_face, face);
        if curr_face.0.contains(&temp.0) && curr_face.1.contains(&temp.1) {
            // We're in the same face, so let's return!
            println!(" Still good.");
            return (temp.0 as usize, temp.1 as usize, self.clone());
        }

        // Otherwise we fell off, and get the next face/direction.
        let next = move_map.get(&(face, self.clone())).unwrap();
        let next_face = faces[next.0].clone();
        println!(" Moving to {:?} ({})", next_face, next.0);

        // Figure out where we're coming from, and map it to the right location on the new face.
        match (self, &next.1) {
            (Direction::Up, Direction::Up) => {
                (temp.0 as usize, *next_face.1.end() as usize, next.1.clone())
            }
            (Direction::Left, Direction::Left) => {
                (*next_face.0.end() as usize, temp.1 as usize, next.1.clone())
            }
            (Direction::Down, Direction::Down) => (
                temp.0 as usize,
                *next_face.1.start() as usize,
                next.1.clone(),
            ),
            (Direction::Right, Direction::Right) => (
                *next_face.0.start() as usize,
                temp.1 as usize,
                next.1.clone(),
            ),
            (Direction::Right, Direction::Left) => {
                println!(
                    "    Right/Left: {:?}",
                    (*next_face.0.end() as usize, temp.1 as usize, next.1.clone())
                );
                (*next_face.0.end() as usize, temp.1 as usize, next.1.clone())
            }
            _ => todo!("Unknown combo: {:?}/{:?}", self, next.1),
        }
    }
}

fn map_line(i: &str) -> IResult<&str, Vec<Cell>> {
    let (input, line) = many1(alt((
        complete::char(' '),
        complete::char('.'),
        complete::char('#'),
    )))(i)?;
    let rv = line
        .iter()
        .map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            ' ' => Cell::OutOfBounds,
            _ => panic!("Unknown cell! {}", c),
        })
        .collect();
    Ok((input, rv))
}

fn map(i: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    let (input, mut lines) = many1(terminated(map_line, line_ending))(i)?;
    let max_length = lines.iter().map(|l| l.len()).max().unwrap();
    for line in lines.iter_mut() {
        let line_len = line.len();
        if line_len < max_length {
            line.extend(iter::repeat(Cell::OutOfBounds).take(max_length - line_len));
            // println!("Extending {} to {} == {}", line_len, max_length, line.len());
        }
    }
    Ok((input, lines))
}

fn forward(i: &str) -> IResult<&str, Move> {
    let (input, value) = complete::u64(i)?;
    Ok((input, Move::Forward(value as usize)))
}

fn turn(i: &str) -> IResult<&str, Move> {
    let (input, value) = alt((complete::char('L'), complete::char('R')))(i)?;
    let rv = match value {
        'L' => Move::Left,
        'R' => Move::Right,
        _ => panic!("Unknown move! {}", value),
    };
    Ok((input, rv))
}

fn moves(i: &str) -> IResult<&str, Vec<Move>> {
    let (input, line) = many1(alt((forward, turn)))(i)?;
    Ok((input, line))
}

fn parser(i: &str) -> IResult<&str, (Vec<Vec<Cell>>, Vec<Move>)> {
    let (input, (map, moves)) = separated_pair(map, line_ending, moves)(i)?;
    Ok((input, (map, moves)))
}

fn parse_data(
    data: &str,
    simple: bool,
) -> (
    Vec<Vec<Cell>>,
    Vec<Move>,
    (usize, usize, Direction),
    Vec<Face>,
    HashMap<(usize, Direction), (usize, Direction)>,
) {
    let (map, moves) = parser(data).unwrap().1;
    let curr = (
        map[0].iter().position(|c| c == &Cell::Empty).unwrap(),
        0usize,
        Direction::Right,
    );
    println!("start: {:?}: {:?}", curr, map[curr.1][curr.0]);
    // figure out the cubes.
    // The size will be the minimum length, getting rid of OutOfBoundss.
    let size = map
        .iter()
        .map(|line| {
            line.iter()
                .filter(|&cell| cell != &Cell::OutOfBounds)
                .count()
        })
        .min()
        .unwrap();
    println!("size = {}", size);
    let faces = map
        .iter()
        .enumerate()
        .step_by(size)
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .step_by(size)
                .map(|(col, cell)| (col, row, cell))
                .filter(|&(_, _, c)| c != &Cell::OutOfBounds)
                .map(|(x, y, _cell)| {
                    (
                        (x as i64)..=((x + size - 1) as i64),
                        (y as i64)..=((y + size - 1) as i64),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("faces: {:?}", faces);
    let move_map = match (size, simple) {
        (4, true) => {
            // Test data!
            // ..0.
            // 123.
            // ..45
            // curr => U, L, D, R
            HashMap::from([
                // 0   => 4U, 0L, 3D, 0R
                ((0, Direction::Up), (4, Direction::Up)),
                ((0, Direction::Left), (0, Direction::Left)),
                ((0, Direction::Down), (3, Direction::Down)),
                ((0, Direction::Right), (0, Direction::Right)),
                // 1   => 1U, 3L, 1D, 2R
                ((1, Direction::Up), (1, Direction::Up)),
                ((1, Direction::Left), (3, Direction::Left)),
                ((1, Direction::Down), (1, Direction::Down)),
                ((1, Direction::Right), (2, Direction::Right)),
                // 2   => 2U, 1L, 2D, 3R
                ((2, Direction::Up), (2, Direction::Up)),
                ((2, Direction::Left), (1, Direction::Left)),
                ((2, Direction::Down), (2, Direction::Down)),
                ((2, Direction::Right), (3, Direction::Right)),
                // 3   => 0U, 2L, 4D, 1R
                ((3, Direction::Up), (0, Direction::Up)),
                ((3, Direction::Left), (2, Direction::Left)),
                ((3, Direction::Down), (4, Direction::Down)),
                ((3, Direction::Right), (1, Direction::Right)),
                // 4   => 3U, 5L, 0D, 5R
                ((4, Direction::Up), (3, Direction::Up)),
                ((4, Direction::Left), (5, Direction::Left)),
                ((4, Direction::Down), (0, Direction::Down)),
                ((4, Direction::Right), (5, Direction::Right)),
                // 5   => 5U, 4L, 5D, 4R
                ((5, Direction::Up), (5, Direction::Up)),
                ((5, Direction::Left), (4, Direction::Left)),
                ((5, Direction::Down), (5, Direction::Down)),
                ((5, Direction::Right), (4, Direction::Right)),
            ])
        }
        (_, true) => {
            // Real data.
            // .01
            // .2.
            // 34.
            // 5..
            // curr => U, L, D, R
            HashMap::from([
                // 0   => 4U, 1L, 2D, 1R
                ((0, Direction::Up), (4, Direction::Up)),
                ((0, Direction::Left), (1, Direction::Left)),
                ((0, Direction::Down), (2, Direction::Down)),
                ((0, Direction::Right), (1, Direction::Right)),
                // 1   => 1U, 0L, 1D, 0R
                ((1, Direction::Up), (1, Direction::Up)),
                ((1, Direction::Left), (0, Direction::Left)),
                ((1, Direction::Down), (1, Direction::Down)),
                ((1, Direction::Right), (0, Direction::Right)),
                // 2   => 0U, 2L, 4D, 2R
                ((2, Direction::Up), (0, Direction::Up)),
                ((2, Direction::Left), (2, Direction::Left)),
                ((2, Direction::Down), (4, Direction::Down)),
                ((2, Direction::Right), (2, Direction::Right)),
                // 3   => 5U, 4L, 5D, 4R
                ((3, Direction::Up), (5, Direction::Up)),
                ((3, Direction::Left), (4, Direction::Left)),
                ((3, Direction::Down), (5, Direction::Down)),
                ((3, Direction::Right), (4, Direction::Right)),
                // 4   => 2U, 3L, 0D, 3R
                ((4, Direction::Up), (2, Direction::Up)),
                ((4, Direction::Left), (3, Direction::Left)),
                ((4, Direction::Down), (0, Direction::Down)),
                ((4, Direction::Right), (3, Direction::Right)),
                // 5   => 3U, 5L, 3D, 5R
                ((5, Direction::Up), (3, Direction::Up)),
                ((5, Direction::Left), (5, Direction::Left)),
                ((5, Direction::Down), (3, Direction::Down)),
                ((5, Direction::Right), (5, Direction::Right)),
            ])
        }
        (3, false) => {
            // Test data!
            // ..0.
            // 123.
            // ..45
            // curr => U, L, D, R
            HashMap::from([
                // 0   => 1D, 2D, 3D, 5L
                ((0, Direction::Up), (1, Direction::Down)),
                ((0, Direction::Left), (2, Direction::Down)),
                ((0, Direction::Down), (3, Direction::Down)),
                ((0, Direction::Right), (5, Direction::Left)),
                // 1   => 0D, 5U, 4U, 2R
                ((1, Direction::Up), (0, Direction::Down)),
                ((1, Direction::Left), (5, Direction::Up)),
                ((1, Direction::Down), (4, Direction::Up)),
                ((1, Direction::Right), (2, Direction::Right)),
                // 2   => 0R, 1L, 4R, 3R
                ((2, Direction::Up), (0, Direction::Right)),
                ((2, Direction::Left), (1, Direction::Left)),
                ((2, Direction::Down), (4, Direction::Right)),
                ((2, Direction::Right), (3, Direction::Right)),
                // 3   => 0U, 2L, 4D, 5D
                ((3, Direction::Up), (0, Direction::Up)),
                ((3, Direction::Left), (2, Direction::Left)),
                ((3, Direction::Down), (4, Direction::Down)),
                ((3, Direction::Right), (5, Direction::Down)),
                // 4   => 3U, 2U, 1U, 5R
                ((4, Direction::Up), (3, Direction::Up)),
                ((4, Direction::Left), (2, Direction::Up)),
                ((4, Direction::Down), (1, Direction::Up)),
                ((4, Direction::Right), (5, Direction::Right)),
                // 5   => 3L, 4L, 1R, 0L
                ((5, Direction::Up), (3, Direction::Left)),
                ((5, Direction::Left), (4, Direction::Left)),
                ((5, Direction::Down), (1, Direction::Right)),
                ((5, Direction::Right), (0, Direction::Left)),
            ])
        }
        (_, false) => {
            // Real data.
            // .01
            // .2.
            // 34.
            // 5..
            // curr => U, L, D, R

            HashMap::from([
                // 0   => 5R, 3R, 2D, 1R
                ((0, Direction::Up), (5, Direction::Right)),
                ((0, Direction::Left), (3, Direction::Right)),
                ((0, Direction::Down), (2, Direction::Down)),
                ((0, Direction::Right), (1, Direction::Right)),
                // 1   => 5U, 0L, 2L, 4L
                ((1, Direction::Up), (5, Direction::Up)),
                ((1, Direction::Left), (0, Direction::Left)),
                ((1, Direction::Down), (2, Direction::Left)),
                ((1, Direction::Right), (4, Direction::Left)),
                // 2   => 0U, 3D, 4D, 1U
                ((2, Direction::Up), (0, Direction::Up)),
                ((2, Direction::Left), (3, Direction::Down)),
                ((2, Direction::Down), (4, Direction::Down)),
                ((2, Direction::Right), (1, Direction::Up)),
                // 3   => 2R, 0R, 5D, 4R
                ((3, Direction::Up), (2, Direction::Right)),
                ((3, Direction::Left), (0, Direction::Right)),
                ((3, Direction::Down), (5, Direction::Down)),
                ((3, Direction::Right), (4, Direction::Right)),
                // 4   => 2U, 3L, 5L, 1L
                ((4, Direction::Up), (2, Direction::Up)),
                ((4, Direction::Left), (3, Direction::Left)),
                ((4, Direction::Down), (5, Direction::Left)),
                ((4, Direction::Right), (1, Direction::Left)),
                // 5   => 3U, 0D, 1D, 4U
                ((5, Direction::Up), (3, Direction::Up)),
                ((5, Direction::Left), (0, Direction::Down)),
                ((5, Direction::Down), (1, Direction::Down)),
                ((5, Direction::Right), (4, Direction::Up)),
            ])
        }
    };
    (map, moves, curr, faces, move_map)
}

fn get_face(x: usize, y: usize, faces: &[Face]) -> Option<usize> {
    faces
        .iter()
        .position(|(face_x, face_y)| face_x.contains(&(x as i64)) && face_y.contains(&(y as i64)))
}

fn process_data_a(data: &str) -> usize {
    let (map, moves, mut curr, faces, move_map) = parse_data(data, true);

    for next in moves {
        curr = next.update(curr, &faces, &map, &move_map);
    }
    println!(
        "row: {} column: {}, value: {:?}/{}",
        curr.1 + 1,
        curr.0 + 1,
        curr.2,
        curr.2.get_value()
    );
    (curr.1 + 1) * 1000 + (curr.0 + 1) * 4 + curr.2.get_value()
}

fn process_data_b(data: &str) -> usize {
    let (map, moves, mut curr, faces, move_map) = parse_data(data, false);

    for next in moves {
        curr = next.update(curr, &faces, &map, &move_map);
    }
    println!(
        "row: {} column: {}, value: {:?}/{}",
        curr.1 + 1,
        curr.0 + 1,
        curr.2,
        curr.2.get_value()
    );
    (curr.1 + 1) * 1000 + (curr.0 + 1) * 4 + curr.2.get_value()
}

//-----------------------------------------------------
// Questions.

q_impl!("22");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"
        )),
        6032
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"
        )),
        5031
    );
}
