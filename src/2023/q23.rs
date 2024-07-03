//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet, VecDeque};

use aoc::util::{point_to_index, Direction, Point2};

static INPUT: &str = include_str!("data/q23.data");

pub struct Input {
    extra: u32,
    horizontal: [[u32; 6]; 6],
    vertical: [[u32; 6]; 6],
}

struct State {
    letter: u8,
    skipped: bool,
    grid: [[u8; 6]; 7],
    convert: [u8; 32],
    result: u32,
}

pub fn parse(input: &str) -> Input {
    let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
    let width = raw[0].len() as i64;
    let height = raw.len() as i64;
    let mut grid = Vec::with_capacity((width * height) as usize);
    raw.iter().for_each(|slice| grid.extend_from_slice(slice));

    // Modify edge of grid to remove the need for boundary checks.
    grid[1] = b'#';
    grid[point_to_index((width - 2, height - 1), width)] = b'#';

    // Move start and end away from edge.
    let start = (1, 1);
    let end = (width - 2, height - 2);

    // Points of interest are start, end and junctions.
    grid[point_to_index(start, width)] = b'P';
    grid[point_to_index(end, width)] = b'P';

    let mut poi = Vec::new();
    poi.push(start);
    poi.push(end);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let position = (x, y);

            if grid[point_to_index(position, width)] != b'#' {
                let neighbors = Direction::all()
                    .iter()
                    .map(|&o| o.move_pos(position, 1, None, None).unwrap())
                    .filter(|&n| grid[point_to_index(n, width)] != b'#')
                    .count();
                if neighbors > 2 {
                    grid[point_to_index(position, width)] = b'P';
                    poi.push(position);
                }
            }
        }
    }

    // BFS to find distances between POIs.
    let mut todo = VecDeque::new();
    let mut edges = HashMap::new();
    let mut weight = HashMap::new();

    for from in poi {
        todo.push_back((from, 0));
        grid[point_to_index(from, width)] = b'#';
        weight.insert((from, from), 0);

        while let Some((position, cost)) = todo.pop_front() {
            for direction in Direction::all() {
                let to = direction.move_pos(position, 1, None, None).unwrap();

                match grid[point_to_index(to, width)] {
                    b'#' => {}
                    b'P' => {
                        edges.entry(from).or_insert(HashSet::new()).insert(to);
                        edges.entry(to).or_insert(HashSet::new()).insert(from);
                        weight.insert((from, to), cost + 1);
                        weight.insert((to, from), cost + 1);
                    }
                    _ => {
                        todo.push_back((to, cost + 1));
                        grid[point_to_index(to, width)] = b'#';
                    }
                }
            }
        }
    }

    // Convert reduced graph to a 6x6 square grid.
    graph_to_grid(start, end, &edges, &weight)
}

fn graph_to_grid(
    start: Point2,
    end: Point2,
    edges: &HashMap<Point2, HashSet<Point2>>,
    weight: &HashMap<(Point2, Point2), u32>,
) -> Input {
    let mut extra = 2;
    extra += edges[&start]
        .iter()
        .map(|&e| weight[&(start, e)])
        .sum::<u32>();
    extra += edges[&end].iter().map(|&e| weight[&(e, end)]).sum::<u32>();

    let mut places = [[(0, 0); 6]; 6];
    let mut horizontal = [[0; 6]; 6];
    let mut vertical = [[0; 6]; 6];

    let mut point = *edges[&start].iter().next().unwrap();
    let mut seen = HashSet::new();
    let mut next_perimeter = |point: Point2| {
        seen.insert(point);
        *edges[&point]
            .iter()
            .find(|&next| edges[next].len() == 3 && !seen.contains(next))
            .unwrap_or(&(0, 0))
    };

    for place in places.iter_mut().take(5) {
        place[0] = point;
        point = next_perimeter(point);
    }

    for x in 1..6 {
        places[5][x] = point;
        point = next_perimeter(point);
    }

    for y in (1..5).rev() {
        places[y][5] = point;
        point = next_perimeter(point);
    }

    for x in (1..5).rev() {
        places[0][x] = point;
        point = next_perimeter(point);
    }

    for y in 1..5 {
        for x in 1..5 {
            let above = places[y - 1][x];
            let left = places[y][x - 1];
            let (&point, _) = edges
                .iter()
                .find(|(k, v)| !seen.contains(k) && v.contains(&above) && v.contains(&left))
                .unwrap();

            places[y][x] = point;
            seen.insert(point);
        }
    }

    places[0][5] = places[0][4];
    places[5][0] = places[5][1];

    for y in 0..6 {
        for x in 0..5 {
            let key = (places[y][x], places[y][x + 1]);
            horizontal[y][x] = *weight.get(&key).unwrap_or(&0);
        }
    }

    for y in 0..5 {
        for x in 0..6 {
            let key = (places[y][x], places[y + 1][x]);
            vertical[y][x] = *weight.get(&key).unwrap_or(&0);
        }
    }

    Input {
        extra,
        horizontal,
        vertical,
    }
}

fn dfs(input: &Input, state: &mut State, mut row: usize, mut col: usize, mut steps: u32) {
    // Wrap around at end of each row.
    if col == 6 {
        // We've reached the bottom right corner.
        if row == 5 {
            state.result = state.result.max(steps);
            return;
        }
        row += 1;
        col = 0;
    }

    if state.grid[row][col] == 0 {
        // Skip only 1 node in each path.
        if !(state.skipped || (row == 5 && col == 5)) {
            state.skipped = true;
            state.grid[row + 1][col] = 0;
            dfs(input, state, row, col + 1, steps);
            state.skipped = false;
        }

        // Create new paths (except on the final row).
        if row < 5 {
            let id = state.letter;
            steps += input.vertical[row][col];

            for end in (col + 1)..6 {
                state.grid[row + 1][end - 1] = 0;
                steps += input.horizontal[row][end - 1];

                if state.grid[row][end] == 0 {
                    state.grid[row + 1][col] = id;
                    state.grid[row + 1][end] = id;
                    let extra = input.vertical[row][end];
                    state.letter += 1;
                    dfs(input, state, row, end + 1, steps + extra);
                    state.letter -= 1;
                } else {
                    state.grid[row + 1][col] = state.convert[state.grid[row][end] as usize];
                    state.grid[row + 1][end] = 0;
                    dfs(input, state, row, end + 1, steps);
                    break;
                }
            }
        }
    } else {
        let index = state.grid[row][col] as usize;
        let id = state.convert[index];

        // Straight down
        if row < 5 || col == 5 {
            state.grid[row + 1][col] = id;
            let extra = input.vertical[row][col];
            dfs(input, state, row, col + 1, steps + extra);
        }

        for end in (col + 1)..6 {
            state.grid[row + 1][end - 1] = 0;
            steps += input.horizontal[row][end - 1];

            if state.grid[row][end] == 0 {
                // Move down only if not final row (except final corner).
                if row < 5 || end == 5 {
                    state.grid[row + 1][end] = id;
                    let extra = input.vertical[row][end];
                    dfs(input, state, row, end + 1, steps + extra);
                }
            } else {
                // Join two path together as long as they are different.
                // (prevent disjoint loops)
                let other = state.convert[state.grid[row][end] as usize];

                if id != other {
                    state.grid[row + 1][end] = 0;
                    state.convert[index] = other;
                    dfs(input, state, row, end + 1, steps);
                    state.convert[index] = id;
                }

                break;
            }
        }
    }
}

fn process_data_a(data: &str) -> u32 {
    let input = parse(data);
    let mut total = [[0; 6]; 6];

    for y in 0..6 {
        for x in 0..6 {
            let left = if x == 0 {
                0
            } else {
                total[y][x - 1] + input.horizontal[y][x - 1]
            };
            let above = if y == 0 {
                0
            } else {
                total[y - 1][x] + input.vertical[y - 1][x]
            };
            total[y][x] = left.max(above);
        }
    }

    assert_eq!(2362, input.extra + total[5][5]);
    input.extra + total[5][5]
}

fn process_data_b(data: &str) -> u32 {
    let input = parse(data);

    let mut state = State {
        letter: 2,
        skipped: false,
        grid: [[0; 6]; 7],
        convert: [0; 32],
        result: 0,
    };

    state.grid[0][0] = 1;

    for i in 0..32 {
        state.convert[i] = i as u8;
    }

    dfs(&input, &mut state, 0, 0, 0);
    assert_eq!(6538, input.extra + state.result);
    input.extra + state.result
}

//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    //     use pretty_assertions::assert_eq;

    //     assert_eq!(
    //         process_data_a(indoc!(
    //             "
    //     #.#####################
    //     #.......#########...###
    //     #######.#########.#.###
    //     ###.....#.>.>.###.#.###
    //     ###v#####.#v#.###.#.###
    //     ###.>...#.#.#.....#...#
    //     ###v###.#.#.#########.#
    //     ###...#.#.#.......#...#
    //     #####.#.#.#######.#.###
    //     #.....#.#.#.......#...#
    //     #.#####.#.#.#########v#
    //     #.#...#...#...###...>.#
    //     #.#.#v#######v###.###v#
    //     #...#.>.#...>.>.#.###.#
    //     #####v#.#.###v#.#.###.#
    //     #.....#...#...#.#.#...#
    //     #.#########.###.#.#.###
    //     #...###...#...#...#.###
    //     ###.###.#.###v#####v###
    //     #...#...#.#.>.>.#.>.###
    //     #.###.###.#.###.#.#v###
    //     #.....###...###...#...#
    //     #####################.#
    // "
    //         )),
    //         94
    //     );
}

#[test]
fn b() {
    //     use pretty_assertions::assert_eq;

    //     assert_eq!(
    //         process_data_b(indoc!(
    //             "
    //     #.#####################
    //     #.......#########...###
    //     #######.#########.#.###
    //     ###.....#.>.>.###.#.###
    //     ###v#####.#v#.###.#.###
    //     ###.>...#.#.#.....#...#
    //     ###v###.#.#.#########.#
    //     ###...#.#.#.......#...#
    //     #####.#.#.#######.#.###
    //     #.....#.#.#.......#...#
    //     #.#####.#.#.#########v#
    //     #.#...#...#...###...>.#
    //     #.#.#v#######v###.###v#
    //     #...#.>.#...>.>.#.###.#
    //     #####v#.#.###v#.#.###.#
    //     #.....#...#...#.#.#...#
    //     #.#########.###.#.#.###
    //     #...###...#...#...#.###
    //     ###.###.#.###v#####v###
    //     #...#...#.#.>.>.#.>.###
    //     #.###.###.#.###.#.#v###
    //     #.....###...###...#...#
    //     #####################.#
    // "
    //         )),
    //         154
    //     );
}
