//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

use num_integer::lcm;

static INPUT: &str = include_str!("data/q24.data");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    Wait,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Goal {
    End,
    Start,
    EndAgain,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    player: (usize, usize),
    goal: Goal,
}

fn inc_player(curr: &State, start: (usize, usize), end: (usize, usize)) -> State {
    let mut rv = curr.clone();
    if rv.player == end && rv.goal == Goal::End {
        // got to the end, go back to the start…
        rv.goal = Goal::Start;
    } else if rv.player == start && rv.goal == Goal::Start {
        // got to the start, go back to the end again…
        rv.goal = Goal::EndAgain;
    }

    rv
}

fn move_blizzards(blizzards: &mut HashSet<(usize, usize, Direction)>, bounds: (usize, usize)) {
    let curr_blizzards = blizzards.clone();
    blizzards.clear();
    for blizzard in &curr_blizzards {
        match blizzard.2 {
            Direction::North => {
                let mut y = blizzard.1 - 1;
                if y == 0 {
                    y = bounds.1 - 1;
                }
                blizzards.insert((blizzard.0, y, blizzard.2));
            }
            Direction::East => {
                let mut x = blizzard.0 + 1;
                if x == bounds.0 {
                    x = 1;
                }
                blizzards.insert((x, blizzard.1, blizzard.2));
            }
            Direction::South => {
                let mut y = blizzard.1 + 1;
                if y == bounds.1 {
                    y = 1;
                }
                blizzards.insert((blizzard.0, y, blizzard.2));
            }
            Direction::West => {
                let mut x = blizzard.0 - 1;
                if x == 0 {
                    x = bounds.0 - 1;
                }
                blizzards.insert((x, blizzard.1, blizzard.2));
            }
            Direction::Wait => panic!("Can't have a waiting blizzard!"),
        }
    }
}

fn get_player_moves(
    curr: &State,
    blizzards: &HashSet<(usize, usize, Direction)>,
    start: (usize, usize),
    end: (usize, usize),
    bounds: (usize, usize),
) -> Vec<State> {
    let mut rv = vec![];
    let blizzards: HashSet<(usize, usize)> = blizzards.iter().map(|&(x, y, _)| (x, y)).collect();

    for dir in &[
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::Wait,
    ] {
        match dir {
            Direction::North => {
                let mut player = curr.player;

                if player == start {
                    continue;
                }
                player.1 -= 1;
                if player == start || (player.1 != 0 && !blizzards.contains(&player)) {
                    rv.push(State {
                        player,
                        goal: curr.goal,
                    })
                }
            }
            Direction::East => {
                let mut player = curr.player;
                if player == start || player == end {
                    continue;
                }
                player.0 += 1;
                if player.0 != bounds.0 && !blizzards.contains(&player) {
                    rv.push(State {
                        player,
                        goal: curr.goal,
                    })
                }
            }
            Direction::South => {
                let mut player = curr.player;
                if player == end {
                    continue;
                }
                player.1 += 1;
                if player == end || (player.1 != bounds.1 && !blizzards.contains(&player)) {
                    rv.push(State {
                        player,
                        goal: curr.goal,
                    })
                }
            }
            Direction::West => {
                let mut player = curr.player;
                if player == start || player == end {
                    continue;
                }
                player.0 -= 1;
                if player.0 != 0 && !blizzards.contains(&player) {
                    rv.push(State {
                        player,
                        goal: curr.goal,
                    })
                }
            }
            Direction::Wait => {
                if !blizzards.contains(&curr.player) {
                    rv.push(curr.clone());
                }
            }
        }
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let mut blizzards = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut bounds = (0, 0);

    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if start == (0, 0) && cell == '.' {
                start = (x, y);
            }

            match cell {
                '^' => {
                    blizzards.insert((x, y, Direction::North));
                }
                '>' => {
                    blizzards.insert((x, y, Direction::East));
                }
                'v' => {
                    blizzards.insert((x, y, Direction::South));
                }
                '<' => {
                    blizzards.insert((x, y, Direction::West));
                }
                '.' => {
                    end = (x, y);
                }
                '#' => {
                    bounds = (x, y);
                }
                _ => {}
            };
        }
    }
    let _cycle = lcm(bounds.0 - 1, bounds.1 - 1);

    let curr = State {
        player: start,
        goal: Goal::End,
    };
    let mut states = vec![curr];

    let mut turn = 0;

    'outer: while !states.is_empty() {
        let mut next_states = vec![];
        turn += 1;

        let mut seen = HashSet::new();
        move_blizzards(&mut blizzards, bounds);

        for curr in states {
            if curr.player == end {
                // Found it!
                rv = turn - 1;
                break 'outer;
            }

            let next = inc_player(&curr, start, end);
            for next in get_player_moves(&next, &blizzards, start, end, bounds) {
                if !seen.contains(&next) {
                    seen.insert(next.clone());
                    next_states.push(next);
                }
            }
        }
        states = next_states;
    }

    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let mut blizzards = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut bounds = (0, 0);

    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if start == (0, 0) && cell == '.' {
                start = (x, y);
            }

            match cell {
                '^' => {
                    blizzards.insert((x, y, Direction::North));
                }
                '>' => {
                    blizzards.insert((x, y, Direction::East));
                }
                'v' => {
                    blizzards.insert((x, y, Direction::South));
                }
                '<' => {
                    blizzards.insert((x, y, Direction::West));
                }
                '.' => {
                    end = (x, y);
                }
                '#' => {
                    bounds = (x, y);
                }
                _ => {}
            };
        }
    }
    let _cycle = lcm(bounds.0 - 1, bounds.1 - 1);

    let curr = State {
        player: start,
        goal: Goal::End,
    };
    let mut states = vec![curr];

    let mut turn = 0;

    'outer: while !states.is_empty() {
        let mut next_states = vec![];
        turn += 1;
        let mut seen = HashSet::new();
        move_blizzards(&mut blizzards, bounds);

        for curr in states {
            if curr.player == end && curr.goal == Goal::EndAgain {
                // Found it!
                rv = turn - 1;
                break 'outer;
            }
            let next = inc_player(&curr, start, end);
            for next in get_player_moves(&next, &blizzards, start, end, bounds) {
                if !seen.contains(&next) {
                    seen.insert(next.clone());
                    next_states.push(next);
                }
            }
        }
        states = next_states;
    }

    // 740 is too low.
    // 859
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("24");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "#.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#
    "
        )),
        18
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "#.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#
    "
        )),
        54
    );
}
