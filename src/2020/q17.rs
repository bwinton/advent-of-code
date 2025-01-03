use std::collections::{HashMap, HashSet};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q17.data");

fn run(world: &HashSet<(isize, isize, isize)>) -> HashSet<(isize, isize, isize)> {
    let mut counts = HashMap::new();
    for cell in world {
        // Check the neighbours.
        for x in cell.0 - 1..=cell.0 + 1 {
            for y in cell.1 - 1..=cell.1 + 1 {
                for z in cell.2 - 1..=cell.2 + 1 {
                    if &(x, y, z) == cell {
                        continue;
                    }
                    let entry = counts.entry((x, y, z)).or_insert(0);
                    *entry += 1;
                }
            }
        }
    }
    counts
        .into_iter()
        .filter_map(|(key, value)| {
            if value == 3 || (world.contains(&key) && value == 2) {
                Some(key)
            } else {
                None
            }
        })
        .collect()
}

fn process_data_a(data: &str) -> usize {
    let mut world: HashSet<(isize, isize, isize)> = HashSet::new();
    for (i, line) in data.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if character == '#' {
                world.insert((i as isize, j as isize, 0));
            }
        }
    }

    for _ in 0..6 {
        world = run(&world);
    }

    world.len()
}

fn check_b(
    world: &HashSet<(isize, isize, isize, isize)>,
    cell: &(isize, isize, isize, isize),
) -> bool {
    let mut sum = 0;
    let curr = world.contains(cell);
    for x in cell.0 - 1..=cell.0 + 1 {
        for y in cell.1 - 1..=cell.1 + 1 {
            for z in cell.2 - 1..=cell.2 + 1 {
                for w in cell.3 - 1..=cell.3 + 1 {
                    if (x, y, z, w) == *cell {
                        continue;
                    }
                    if world.contains(&(x, y, z, w)) {
                        sum += 1;
                    }
                }
            }
        }
    }
    if curr { sum == 2 || sum == 3 } else { sum == 3 }
}

fn run_b(world: &HashSet<(isize, isize, isize, isize)>) -> HashSet<(isize, isize, isize, isize)> {
    let mut rv = HashSet::new();
    for cell in world {
        // Check the neighbours.
        for x in cell.0 - 1..=cell.0 + 1 {
            for y in cell.1 - 1..=cell.1 + 1 {
                for z in cell.2 - 1..=cell.2 + 1 {
                    for w in cell.3 - 1..=cell.3 + 1 {
                        if &(x, y, z, w) == cell {
                            continue;
                        }
                        if check_b(world, &(x, y, z, w)) {
                            rv.insert((x, y, z, w));
                        }
                    }
                }
            }
        }
    }
    rv
}
fn process_data_b(data: &str) -> usize {
    let mut world: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    for (i, line) in data.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            if character == '#' {
                world.insert((i as isize, j as isize, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        world = run_b(&world);
    }

    world.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("17");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            ".#.
..#
###
"
        ),
        112
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            ".#.
..#
###"
        ),
        848
    );
}
