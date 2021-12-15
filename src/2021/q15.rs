use std::{cmp::Ordering, collections::BinaryHeap};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q15.data");

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    path: Vec<(usize, usize)>,
    risk: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.risk.partial_cmp(&self.risk) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        other.path.len().partial_cmp(&self.path.len())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.risk.cmp(&self.risk) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        other.path.len().cmp(&self.path.len())
    }
}

fn next(x: i32, y: i32, curr: &State, map: &[Vec<usize>]) -> Option<State> {
    if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
        None
    } else {
        let position = (x as usize, y as usize);
        let mut path = curr.path.clone();
        path.push(position);
        let risk = curr.risk + map[position.1][position.0];
        Some(State { path, risk })
    }
}

fn get_risk(map: &[Vec<usize>], seen: &mut [Vec<usize>]) -> usize {
    seen[0][0] = 0;
    let start = State {
        path: vec![(0, 0)],
        risk: 0,
    };
    let end = (map[0].len() - 1, map.len() - 1);

    let mut rv = State {
        path: vec![],
        risk: usize::MAX,
    };
    let mut states = BinaryHeap::new();
    states.push(start);
    'outer: while !states.is_empty() {
        let curr = states.pop().unwrap();
        let &(x, y) = curr.path.last().unwrap();
        seen[y][x] = curr.risk;
        for (delta_x, delta_y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if let Some(state) = next(x as i32 + delta_x, y as i32 + delta_y, &curr, map) {
                let position = *state.path.last().unwrap();
                if position == end && seen[position.1][position.0] > state.risk {
                    rv = state;
                    break 'outer;
                }
                if seen[position.1][position.0] > state.risk {
                    seen[position.1][position.0] = state.risk;
                    states.push(state);
                }
            }
        }
    }
    rv.risk
}

fn process_data_a(data: &str) -> usize {
    let mut map = vec![];
    let mut seen = vec![];

    for line in data.lines() {
        // Do something
        let row: Vec<usize> = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        map.push(row);
        let row: Vec<usize> = line.chars().map(|_| usize::MAX).collect();
        seen.push(row);
    }
    get_risk(&map, &mut seen)
}

fn process_data_b(data: &str) -> usize {
    let mut map = vec![];
    let mut seen = vec![];

    for line in data.lines() {
        // Do something
        let mut full_row = vec![];
        let row: Vec<usize> = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        for i in 0..5 {
            full_row.extend(row.iter().map(|x| (x + i - 1) % 9 + 1));
        }
        map.push(full_row);

        let full_row: Vec<usize> = map[0].iter().map(|_| usize::MAX).collect();
        seen.push(full_row);
    }

    let mut full_map = vec![];
    for i in 0..5 {
        for row in &map {
            let full_row: Vec<usize> = row.iter().map(|x| (x + i - 1) % 9 + 1).collect();
            full_map.push(full_row);
            seen.push(seen[0].clone());
        }
    }
    get_risk(&full_map, &mut seen)
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581"
        )),
        40
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581"
        )),
        315
    );
}
