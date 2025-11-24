//-----------------------------------------------------
// Setup.

use std::collections::BTreeSet;

use aoc::util::{Direction, Point2};
use nom::{
    IResult, Parser,
    character::complete::{newline, one_of},
    multi::{many0, many1},
    sequence::terminated,
};

static INPUT: &str = include_str!("data/q15.data");

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Object {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

fn map(i: &str) -> IResult<&str, Vec<Vec<Object>>> {
    let (input, map) = many0(terminated(many1(one_of("#O[].@")), newline)).parse(i)?;
    let map = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| match c {
                    '#' => Object::Wall,
                    'O' => Object::Box,
                    '[' => Object::BoxLeft,
                    ']' => Object::BoxRight,
                    '.' => Object::Empty,
                    '@' => Object::Robot,
                    _ => panic!("Unknown cell {:?}!", c),
                })
                .collect()
        })
        .collect();
    Ok((input, map))
}

fn moves(i: &str) -> IResult<&str, Vec<Direction>> {
    let (input, moves) = many0(one_of("^>v<\n")).parse(i)?;
    let moves = moves
        .into_iter()
        .filter_map(|c| match c {
            '^' => Some(Direction::North),
            '>' => Some(Direction::East),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            '\n' => None,
            _ => panic!("Unknown direction {:?}!", c),
        })
        .collect();
    Ok((input, moves))
}

fn parser(i: &str) -> IResult<&str, (Vec<Vec<Object>>, Vec<Direction>)> {
    let (input, (map, _, moves)) = (map, newline, moves).parse(i)?;
    Ok((input, (map, moves)))
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Object>], robot: Point2) {
    for (y, row) in map.iter().cloned().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if robot == (x as i64, y as i64) {
                print!("@");
                if cell != Object::Empty {
                    panic!("Found robot on top of {:?} at {:?}!!!", cell, robot);
                }
                continue;
            }
            match cell {
                Object::Wall => print!("#"),
                Object::Box => print!("O"),
                Object::BoxLeft => print!("["),
                Object::BoxRight => print!("]"),
                Object::Empty => print!("."),
                Object::Robot => print!("@"),
            }
        }
        println!();
    }
    println!();
}

fn move_box_horizontal(
    first_box: (i64, i64),
    direction: Direction,
    min: Option<(i64, i64)>,
    max: Option<(i64, i64)>,
    map: &mut [Vec<Object>],
    robot: &mut (i64, i64),
    next: (i64, i64),
) {
    let mut box_next = Some(first_box);
    loop {
        box_next = direction.move_pos(box_next.unwrap(), 1, min, max);
        if box_next.is_some() {
            let box_next = box_next.unwrap();
            let square = map[box_next.1 as usize][box_next.0 as usize];
            if square == Object::Wall {
                // We can't move this line of boxes, so continue.
                return;
            } else if square == Object::BoxLeft || square == Object::BoxRight {
                // We've found another box, let's keep on going.
                continue;
            } else {
                // We've found an empty space!
                map[first_box.1 as usize][first_box.0 as usize] = Object::Empty;
                let mut next = Some(first_box);
                let mut front = direction == Direction::East;
                loop {
                    next = direction.move_pos(next.unwrap(), 1, min, max);
                    let cell = next.unwrap();
                    map[cell.1 as usize][cell.0 as usize] = if front {
                        Object::BoxLeft
                    } else {
                        Object::BoxRight
                    };
                    front = !front;
                    if cell == box_next {
                        break;
                    }
                }
                *robot = first_box;
                break;
            }
        } else {
            panic!("Ran out of map at {:?} -> {:?}", next, direction)
        }
    }
}

fn move_boxes_vertical(
    first_box: (i64, i64),
    direction: Direction,
    min: Option<(i64, i64)>,
    max: Option<(i64, i64)>,
    map: &mut [Vec<Object>],
    robot: &mut (i64, i64),
    next: (i64, i64),
) {
    let square = map[first_box.1 as usize][first_box.0 as usize];
    let mut line = vec![];
    let mut boxes_to_push = BTreeSet::new();
    if square == Object::BoxLeft {
        boxes_to_push.insert((first_box.1, first_box.0));
        line.push(first_box.0);
        line.push(first_box.0 + 1);
    } else if square == Object::BoxRight {
        boxes_to_push.insert((first_box.1, first_box.0 - 1));
        line.push(first_box.0 - 1);
        line.push(first_box.0);
    };
    let mut box_next = Some(first_box);

    loop {
        box_next = direction.move_pos(box_next.unwrap(), 1, min, max);
        if box_next.is_some() {
            let box_next = box_next.unwrap();
            let mut found_box = false;
            let squares: Vec<Object> = line
                .iter()
                .map(|c| map[box_next.1 as usize][*c as usize])
                .collect();
            let mut new_line = vec![];

            if squares.iter().any(|&square| square == Object::Wall) {
                // We can't move this line of boxes, so continue.
                return;
            }
            for (i, &square) in squares.iter().enumerate() {
                if square == Object::BoxLeft {
                    boxes_to_push.insert((box_next.1, line[i]));
                    new_line.push(line[i]);
                    new_line.push(line[i] + 1);
                    found_box = true;
                } else if square == Object::BoxRight {
                    boxes_to_push.insert((box_next.1, line[i] - 1));
                    new_line.push(line[i] - 1);
                    new_line.push(line[i]);
                    found_box = true;
                };
            }
            if !found_box {
                // We've found an empty space!
                if direction == Direction::North {
                    for next_box in boxes_to_push {
                        let (y, x) = (next_box.0 as usize, next_box.1 as usize);
                        map[y][x] = Object::Empty;
                        map[y][x + 1] = Object::Empty;
                        map[y - 1][x] = Object::BoxLeft;
                        map[y - 1][x + 1] = Object::BoxRight;
                    }
                } else {
                    for next_box in boxes_to_push.iter().rev() {
                        let (y, x) = (next_box.0 as usize, next_box.1 as usize);
                        map[y][x] = Object::Empty;
                        map[y][x + 1] = Object::Empty;
                        map[y + 1][x] = Object::BoxLeft;
                        map[y + 1][x + 1] = Object::BoxRight;
                    }
                }
                *robot = first_box;
                break;
            }
            line = new_line;
        } else {
            panic!("Ran out of map at {:?} -> {:?}", next, direction)
        }
    }
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (mut map, moves) = parser(data).unwrap().1;

    let min = Some((0, 0));
    let max = Some((map[0].len() as i64, map.len() as i64));
    let mut robot = (0, 0);

    'outer: for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == Object::Robot {
                robot = (x as i64, y as i64);
                map[y][x] = Object::Empty;
                break 'outer;
            }
        }
    }
    'outer: for direction in moves {
        if let Some(next) = direction.move_pos(robot, 1, min, max) {
            // Check to see what's there…
            let square = map[next.1 as usize][next.0 as usize];
            if square == Object::Wall {
                // If it's a wall, don't move.
            } else if square == Object::Empty {
                // If it's empty, update your position!
                robot = next;
            } else {
                // Otherwise, it's a box, so try to move the box.
                let first_box = next;
                let mut box_next = Some(first_box);
                loop {
                    box_next = direction.move_pos(box_next.unwrap(), 1, min, max);
                    if box_next.is_some() {
                        let box_next = box_next.unwrap();
                        let square = map[box_next.1 as usize][box_next.0 as usize];
                        if square == Object::Wall {
                            // We can't move this line of boxes, so continue.
                            continue 'outer;
                        } else if square == Object::Box {
                            // We've found another box, let's keep on going.
                            continue;
                        } else {
                            // We've found an empty space!
                            map[box_next.1 as usize][box_next.0 as usize] = Object::Box;
                            map[first_box.1 as usize][first_box.0 as usize] = Object::Empty;
                            robot = next;
                            break;
                        }
                    } else {
                        panic!("Ran out of map at {:?} -> {:?}", next, direction)
                    }
                }
                // Check the next of the box, and so on until you hit empty space or a wall.
            }
        }
    }

    for (y, row) in map.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell == Object::Box {
                rv += 100 * y + x;
            }
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let data = data
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");

    let (mut map, moves) = parser(&data).unwrap().1;
    let min = Some((0, 0));
    let max = Some((map[0].len() as i64, map.len() as i64));
    let mut robot = (0, 0);

    'outer: for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == Object::Robot {
                robot = (x as i64, y as i64);
                map[y][x] = Object::Empty;
                break 'outer;
            }
        }
    }

    // print_map(&map, robot);

    for direction in moves {
        if let Some(next) = direction.move_pos(robot, 1, min, max) {
            // Check to see what's there…
            let square = map[next.1 as usize][next.0 as usize];
            if square == Object::Wall {
                // If it's a wall, don't move.
            } else if square == Object::Empty {
                // If it's empty, update your position!
                robot = next;
            } else {
                // If it's a box, try to move the box(es).
                let first_box = next;
                if direction == Direction::East || direction == Direction::West {
                    move_box_horizontal(first_box, direction, min, max, &mut map, &mut robot, next);
                } else {
                    move_boxes_vertical(first_box, direction, min, max, &mut map, &mut robot, next);
                }
            }
        }
        // print_map(&map, robot);
    }

    // print_map(&map);

    for (y, row) in map.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell == Object::BoxLeft {
                rv += 100 * y + x;
            }
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    <^^>>>vv<v>>v<<
    "
        )),
        2028
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "
        )),
        10092
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "
        )),
        9021
    );
}
