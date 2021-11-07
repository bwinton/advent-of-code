//-----------------------------------------------------
// Setup.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

static INPUT: &str = "4002,5,746";

#[derive(Clone, Debug)]
struct Path {
    x: i32,
    y: i32,
    time: usize,
    item: char,
    _target: (i32, i32),
}

impl Ord for Path {
    fn cmp(&self, other: &Path) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        // self.time == other.time
        self.time == other.time
    }
}

impl Eq for Path {}

impl Path {
    fn switch_item(&self, item: char, indices: &mut HashMap<(i32, i32), i32>) -> Option<Self> {
        match indices[&(self.x, self.y)] % 3 {
            0 => {
                // In rocky regions, You cannot use neither
                if item == ' ' {
                    return None;
                }
            }
            1 => {
                // In wet regions, You cannot use the torch
                if item == 't' {
                    return None;
                }
            }
            2 => {
                // In narrow regions, You cannot use the climbing gear
                if item == 'c' {
                    return None;
                }
            }
            _ => unreachable!(),
        };

        let mut rv = self.clone();
        rv.item = item;
        rv.time += 7;
        Some(rv)
    }

    fn switch_position(
        &self,
        x: i32,
        y: i32,
        indices: &mut HashMap<(i32, i32), i32>,
        max: &mut (i32, i32),
        depth: i32,
    ) -> Option<Self> {
        if x < 0 || y < 0 {
            return None;
        }

        while x > max.0 {
            max.0 += 1;
            indices.insert((max.0, 0), (max.0 * 16807 + depth) % 20183);
            for y in 1..=max.1 {
                indices.insert(
                    (max.0, y),
                    (indices[&(max.0 - 1, y)] * indices[&(max.0, y - 1)] + depth) % 20183,
                );
            }
        }
        while y > max.1 {
            max.1 += 1;
            indices.insert((0, max.1), (max.1 * 48271 + depth) % 20183);
            for x in 1..=max.0 {
                indices.insert(
                    (x, max.1),
                    (indices[&(x - 1, max.1)] * indices[&(x, max.1 - 1)] + depth) % 20183,
                );
            }
        }

        match indices[&(x, y)] % 3 {
            0 => {
                // In rocky regions, You cannot use neither
                if self.item == ' ' {
                    return None;
                }
            }
            1 => {
                // In wet regions, You cannot use the torch
                if self.item == 't' {
                    return None;
                }
            }
            2 => {
                // In narrow regions, You cannot use the climbing gear
                if self.item == 'c' {
                    return None;
                }
            }
            _ => unreachable!(),
        };

        let mut rv = self.clone();
        rv.x = x;
        rv.y = y;
        rv.time += 1;
        Some(rv)
    }

    fn get_moves(
        &self,
        indices: &mut HashMap<(i32, i32), i32>,
        max: &mut (i32, i32),
        depth: i32,
    ) -> Vec<Self> {
        let mut rv = vec![];

        match self.item {
            't' => {
                if let Some(next) = self.switch_item('c', indices) {
                    rv.push(next);
                }
                if let Some(next) = self.switch_item(' ', indices) {
                    rv.push(next);
                }
            }
            'c' => {
                if let Some(next) = self.switch_item(' ', indices) {
                    rv.push(next);
                }
                if let Some(next) = self.switch_item('t', indices) {
                    rv.push(next);
                }
            }
            ' ' => {
                if let Some(next) = self.switch_item('t', indices) {
                    rv.push(next);
                }
                if let Some(next) = self.switch_item('c', indices) {
                    rv.push(next);
                }
            }
            _ => unreachable!(),
        }

        if let Some(next) = self.switch_position(self.x - 1, self.y, indices, max, depth) {
            rv.push(next);
        }
        if let Some(next) = self.switch_position(self.x + 1, self.y, indices, max, depth) {
            rv.push(next);
        }
        if let Some(next) = self.switch_position(self.x, self.y - 1, indices, max, depth) {
            rv.push(next);
        }
        if let Some(next) = self.switch_position(self.x, self.y + 1, indices, max, depth) {
            rv.push(next);
        }
        rv
    }
}

fn process_data_a(data: &str) -> i32 {
    let values: Vec<i32> = data.split(',').map(|x| x.parse().unwrap()).collect();
    let depth = values[0];
    let target = (values[1], values[2]);

    let mut indices = HashMap::new();
    indices.insert((0, 0), depth % 20183);

    for x in 1..=target.0 {
        indices.insert((x, 0), (x * 16807 + depth) % 20183);
    }

    for y in 1..=target.1 {
        indices.insert((0, y), (y * 48271 + depth) % 20183);
    }

    for x in 1..=target.0 {
        for y in 1..=target.1 {
            indices.insert(
                (x, y),
                (indices[&(x - 1, y)] * indices[&(x, y - 1)] + depth) % 20183,
            );
        }
    }

    indices.insert(target, 0);

    indices.values().map(|v| v % 3).sum()
}

fn process_data_b(data: &str) -> usize {
    let values: Vec<i32> = data.split(',').map(|x| x.parse().unwrap()).collect();
    let depth = values[0];
    let target = (values[1], values[2]);

    let mut indices = HashMap::new();
    indices.insert((0, 0), depth % 20183);

    for x in 1..=target.0 {
        indices.insert((x, 0), (x * 16807 + depth) % 20183);
    }

    for y in 1..=target.1 {
        indices.insert((0, y), (y * 48271 + depth) % 20183);
    }

    for x in 1..=target.0 {
        for y in 1..=target.1 {
            indices.insert(
                (x, y),
                (indices[&(x - 1, y)] * indices[&(x, y - 1)] + depth) % 20183,
            );
        }
    }

    indices.insert(target, 0);

    let mut max = target;

    let mut next = BinaryHeap::new();
    next.push(Path {
        x: 0,
        y: 0,
        time: 0,
        item: 't',
        _target: target,
    });
    let mut seen = HashSet::new();
    let mut upcoming = HashMap::new();
    upcoming.insert((0, 0, 't'), 0);

    while !next.is_empty() {
        let curr = next.pop().unwrap();
        seen.insert((curr.x, curr.y, curr.item));

        if (curr.x, curr.y) == target && curr.item == 't' {
            // We made it!
            return curr.time;
        }
        for path in curr.get_moves(&mut indices, &mut max, depth) {
            let key = (path.x, path.y, path.item);
            if !seen.contains(&key) && *upcoming.get(&key).unwrap_or(&usize::MAX) > path.time {
                upcoming.insert(key, path.time);
                next.push(path);
            }
        }
    }

    unreachable!();
}

//-----------------------------------------------------
// Questions.

q_impl!("22");

#[test]
fn a() {
    assert_eq!(process_data_a("510,10,10"), 114);
}

#[test]
fn b() {
    assert_eq!(process_data_b("510,10,10"), 45);
}
