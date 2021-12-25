//-----------------------------------------------------
// Setup.

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
};

static INPUT: &str = include_str!("data/q23.data");

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Invalid,
    Wall,
    Hall,
    Room,
}

type Position = (usize, usize);
type Amphipods = [Vec<(usize, usize)>; 4];
const EMPTY: Vec<(usize, usize)> = vec![];

fn parse_data(data: &str) -> (Vec<Vec<Cell>>, Vec<Position>, Amphipods) {
    let mut board = vec![];
    let mut rooms = vec![];

    let mut amphipods = [EMPTY; 4];
    let a = 'A'.to_digit(16).unwrap();
    for line in data.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                '#' => Cell::Wall,
                '.' | 'A' | 'B' | 'C' | 'D' => {
                    if c != '.' {
                        let pod = &mut amphipods[(c.to_digit(16).unwrap() - a) as usize];
                        pod.push((row.len(), board.len()));
                    }
                    if board.len() == 1 {
                        Cell::Hall
                    } else {
                        if rooms.len() < 4 {
                            rooms.push((row.len(), board.len()));
                        }
                        Cell::Room
                    }
                }
                ' ' => Cell::Invalid,
                _ => panic!("Unknown Char! {}", c),
            });
        }
        board.push(row);
    }
    (board, rooms, amphipods)
}

fn get_hallways(
    board: &[Vec<Cell>],
    rooms: &[Position],
    amphipods: &Amphipods,
    i: usize,
    kind: usize,
) -> Vec<Position> {
    let mut rv = vec![];
    let positions: Vec<Position> = amphipods.iter().flatten().cloned().collect();
    let siblings = &amphipods[kind];
    let (x, mut y) = siblings[i];

    // If we're not in a room, skip it!
    if board[y][x] != Cell::Room {
        return rv;
    }

    // If there's someone above us, skip it!
    if positions.contains(&(x, y - 1)) {
        return rv;
    }

    // See if we're in the stack in our room…
    let (room_x, room_y) = rooms[kind];
    let bottom = room_y + amphipods[kind].len() - 1;
    for j in 0..amphipods[kind].len() {
        if (x, y) == (room_x, bottom - j) {
            // We got to us, so skip the rest of the checks!
            return rv;
        }
        if siblings.contains(&(room_x, bottom - j)) {
            // A sibling is in the stack
            continue;
        } else if positions.contains(&(room_x, bottom - j)) {
            // Someone is here, and it's not our sibling…
            break;
        }
    }

    // Walk up to the hallway…
    y = room_y - 1;

    let direction: isize = if room_x > x { 1 } else { -1 };
    // Now we're in a hallway, so move towards our room.
    let mut closer_x = x;
    while board[y][(closer_x as isize + direction) as usize] == Cell::Hall {
        closer_x = (closer_x as isize + direction) as usize;

        if positions.contains(&(closer_x, y)) {
            break;
        } else if board[y + 1][closer_x] == Cell::Room {
            continue;
        }
        rv.push((closer_x, y));
    }

    // Or we can move away from our room.
    let mut further_x = x;
    while board[y][(further_x as isize - direction) as usize] == Cell::Hall {
        further_x = (further_x as isize - direction) as usize;
        if positions.contains(&(further_x, y)) {
            break;
        } else if board[y + 1][further_x] == Cell::Room {
            continue;
        }
        rv.push((further_x, y));
    }

    rv
}

fn get_rooms(
    board: &[Vec<Cell>],
    rooms: &[Position],
    amphipods: &Amphipods,
    i: usize,
    kind: usize,
) -> Option<Position> {
    let mut others = amphipods
        .iter()
        .enumerate()
        .map(|(kind, values)| values.iter().map(move |&(x, y)| ((x, y), kind)))
        .flatten()
        .collect::<HashMap<_, _>>();
    let (x, y) = amphipods[kind][i];
    others.remove(&(x, y));

    // If we're not in a hall, skip it!
    if board[y][x] != Cell::Hall {
        return None;
    }

    // Now we're in a hallway, so pick our room.
    let (room_x, room_y) = rooms[kind];

    // Then try to move to the room.
    let (src, target) = if room_x < x { (room_x, x) } else { (x, room_x) };
    for x in src..=target {
        if others.contains_key(&(x, y)) {
            return None;
        }
    }

    let bottom = room_y + amphipods[kind].len() - 1;
    // Now move as far down the room as we can!
    for j in 0..amphipods[kind].len() {
        if let Some(occupant) = others.get(&(room_x, bottom - j)) {
            // Someone's in the second spot!
            if *occupant == kind {
                // But they're one of us, so we can sit in the next spot up!
                continue;
            } else {
                // They're someone else, so we can't go to this room…
                return None;
            }
        } else {
            // We found an empty slot!
            return Some((room_x, bottom - j));
        }
    }

    panic!("Didn't expect to get here!!!");
}

fn get_cost(amphipod_type: usize) -> isize {
    match amphipod_type {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        _ => panic!("Unknown Amphipod Type! {}", amphipod_type),
    }
}

fn get_moves(
    board: &[Vec<Cell>],
    rooms: &[Position],
    energy: usize,
    amphipods: &Amphipods,
) -> Vec<(usize, Amphipods)> {
    let mut rv = vec![];

    // Kinds of moves:
    // Amphipods move from hallways into rooms. (Prioritize this, cause it gets us closer!)
    for (kind, group) in amphipods.iter().enumerate() {
        for (i, &(x, y)) in group.iter().enumerate() {
            if let Some((room_x, room_y)) = get_rooms(board, rooms, amphipods, i, kind) {
                let mut amphipods = amphipods.clone();
                let (x, y) = (x as isize, y as isize);
                let (room_x, room_y) = (room_x as isize, room_y as isize);
                let cost = ((x - room_x).abs() + (y - room_y).abs()) * get_cost(kind);
                amphipods[kind][i] = (room_x as usize, room_y as usize);
                rv.push((energy + cost as usize, amphipods));
            }
        }
    }

    // Amphipods move from rooms into hallways.
    for (kind, group) in amphipods.iter().enumerate() {
        for (i, &(x, y)) in group.iter().enumerate() {
            for (hall_x, hall_y) in get_hallways(board, rooms, amphipods, i, kind) {
                let mut amphipods = amphipods.clone();
                let (x, y) = (x as isize, y as isize);
                let (hall_x, hall_y) = (hall_x as isize, hall_y as isize);
                let cost = ((x - hall_x).abs() + (y - hall_y).abs()) * get_cost(kind);
                amphipods[kind][i] = (hall_x as usize, hall_y as usize);
                rv.push((energy + cost as usize, amphipods));
            }
        }
    }

    rv
}

#[allow(dead_code)]
fn print_cell(cell: Cell, position: Position, amphipods: &Amphipods) -> char {
    for (i, kind) in amphipods.iter().enumerate() {
        if kind
            .iter()
            .map(|(x0, y0)| (*x0, *y0))
            .any(|x| x == position)
        {
            return match i {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                3 => 'D',
                _ => panic!("Printing unknown Amphipod Type! {}", i),
            };
        }
    }
    match cell {
        Cell::Invalid => ' ',
        Cell::Wall => '#',
        Cell::Hall | Cell::Room => '.',
    }
}

#[allow(dead_code)]
fn print_board(board: &[Vec<Cell>], amphipods: &Amphipods) {
    for (j, row) in board.iter().enumerate() {
        print!("  ");
        for (i, &cell) in row.iter().enumerate() {
            print!("{}", print_cell(cell, (i, j), amphipods));
        }
        println!();
        if j >= 4 {
            break;
        }
    }
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (board, rooms, amphipods) = parse_data(data);

    let mut win_state = [EMPTY; 4];
    for i in 0..amphipods.len() {
        let room = rooms[i];
        win_state[i] = iter::repeat(room.0)
            .zip(room.1..room.1 + amphipods[i].len())
            .collect();
    }
    let mut states = BinaryHeap::new();
    states.push(Reverse((0, amphipods, vec![])));
    let mut seen = HashSet::new();
    let mut _prev_energy = 0;
    while !states.is_empty() && rv == 0 {
        let Reverse((energy, amphipods, mut prev)) = states.pop().unwrap();
        if seen.contains(&amphipods) {
            continue;
        }
        if amphipods == win_state {
            rv = energy;
            break;
        }

        prev.push((amphipods.clone(), energy));
        seen.insert(amphipods.clone());
        for (energy, mut next) in get_moves(&board, &rooms, energy, &amphipods) {
            // Sort the values for better seen checking.
            for temp in next.iter_mut() {
                temp.sort_unstable();
            }

            if !seen.contains(&next) {
                states.push(Reverse((energy, next, prev.clone())));
            }
        }
    }

    rv
}

fn process_data_b(data: &str) -> usize {
    let mut temp = vec![];
    for (i, line) in data.lines().enumerate() {
        temp.push(line);
        if i == 2 {
            temp.push("  #D#C#B#A#");
            temp.push("  #D#B#A#C#");
        }
    }
    let data = temp.join("\n");

    process_data_a(&data)
}

//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    let (_, rooms, amphipods) = parse_data(indoc!(
        "#############
    #...........#
    ###A#C#B#D###
      #A#C#B#D#
      #########
    "
    ));
    let mut win_state = [EMPTY; 4];
    for i in 0..amphipods.len() {
        let room = rooms[i];
        win_state[i] = iter::repeat(room.0)
            .zip(room.1..room.1 + amphipods[i].len())
            .collect();
    }
    assert_ne!(win_state, amphipods);

    // let (_, _, amphipods) = parse_data(indoc!(
    //     "#############
    // #...........#
    // ###A#B#C#D###
    //   #A#B#C#D#
    //   #########
    // "
    // ));
    // assert_eq!(win_state, amphipods);

    // assert_eq!(
    //     process_data_a(indoc!(
    //         "#############
    // #...........#
    // ###B#A#C#D###
    //   #A#B#C#D#
    //   #########
    // "
    //     )),
    //     46
    // );

    // assert_eq!(
    //     process_data_a(indoc!(
    //         "#############
    // #...........#
    // ###B#A#C#D###
    //   #A#B#C#D#
    //   #A#B#C#D#
    //   #A#B#C#D#
    //   #########
    // "
    //     )),
    //     46
    // );

    assert_eq!(
        process_data_a(indoc!(
            "#############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #########
    "
        )),
        12521
    );

    assert_eq!(
        process_data_a(indoc!(
            "#############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #A#B#C#D#
      #########
    "
        )),
        12521
    );

    assert_eq!(
        process_data_a(indoc!(
            "#############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #A#B#C#D#
      #A#B#C#D#
      #########
    "
        )),
        12521
    );
}

#[test]
fn b() {
    let (_, rooms, amphipods) = parse_data(indoc!(
        "#############
    #...........#
    ###A#C#B#D###
      #A#C#B#D#
      #A#C#B#D#
      #A#C#B#D#
      #########
    "
    ));
    let mut win_state = [EMPTY; 4];
    for i in 0..amphipods.len() {
        let room = rooms[i];
        win_state[i] = iter::repeat(room.0)
            .zip(room.1..room.1 + amphipods[i].len())
            .collect();
    }

    assert_ne!(win_state, amphipods);
    let (_, _, amphipods) = parse_data(indoc!(
        "#############
    #...........#
    ###A#B#C#D###
      #A#B#C#D#
      #A#B#C#D#
      #A#B#C#D#
      #########
    "
    ));
    assert_eq!(win_state, amphipods);

    assert_eq!(
        process_data_b(indoc!(
            "#############
    #...........#
    ###B#C#B#D###
      #A#D#C#A#
      #########
    "
        )),
        44169
    );
}
