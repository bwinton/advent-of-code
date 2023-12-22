//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet, VecDeque};

use aoc::util::Point3;

static INPUT: &str = include_str!("data/q22.data");

#[derive(Clone, Debug)]
struct Block {
    start: Point3,
    end: Point3,
    supported_by: HashSet<usize>,
    supports: HashSet<usize>,
}

impl Block {
    fn new(start: (i64, i64, i64), end: (i64, i64, i64)) -> Block {
        Self {
            start,
            end,
            supported_by: HashSet::default(),
            supports: HashSet::default(),
        }
    }
}

fn parse(data: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = vec![];
    for line in data.lines() {
        let (start, end) = line.split_once('~').unwrap();
        let start = start
            .split(',')
            .map(|value| value.parse().unwrap())
            .collect::<Vec<i64>>();
        let start = (start[0], start[1], start[2]);
        let end = end
            .split(',')
            .map(|value| value.parse().unwrap())
            .collect::<Vec<i64>>();
        let end = (end[0], end[1], end[2]);
        blocks.push(Block::new(start, end));
    }

    // Make sure to drop lower blocks first.
    blocks.sort_by_cached_key(|block| {
        (
            block.start.2.min(block.end.2),
            block.start.0.min(block.end.0),
            block.start.1.min(block.end.1),
        )
    });
    blocks
}

fn drop_blocks(blocks: &mut [Block]) {
    let mut zs = HashMap::new();
    for (i, block) in blocks.iter_mut().enumerate() {
        let min_x = block.start.0.min(block.end.0);
        let max_x = block.start.0.max(block.end.0);
        let min_y = block.start.1.min(block.end.1);
        let max_y = block.start.1.max(block.end.1);
        let min_z = block.start.2.min(block.end.2);
        let mut max_z = block.start.2.max(block.end.2);
        if min_z == 1 {
            // We're at the bottom, so update the zs with our height!
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    zs.insert((x, y), (max_z, Some(i)));
                }
            }
            continue;
        }
        // Otherwise, find the largest z that this block covers:
        let mut top_z = HashSet::new();
        top_z.insert((0, None));
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let high = top_z.iter().next().unwrap().0;
                if let Some(&(z, index)) = zs.get(&(x, y)) {
                    match z.cmp(&high) {
                        std::cmp::Ordering::Less => {}
                        std::cmp::Ordering::Equal => {
                            top_z.insert((z, index));
                        }
                        std::cmp::Ordering::Greater => {
                            top_z.clear();
                            top_z.insert((z, index));
                        }
                    }
                }
            }
        }
        // We now know where this block needs to fall to…
        let high = top_z.iter().next().unwrap().0;
        if high < min_z - 1 {
            let drop = min_z - 1 - high;
            block.start.2 -= drop;
            block.end.2 -= drop;
            max_z -= drop;
        }
        for support in top_z.into_iter().filter(|&(_, index)| index.is_some()) {
            block.supported_by.insert(support.1.unwrap());
        }
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                zs.insert((x, y), (max_z, Some(i)));
            }
        }
    }
    for i in 0..blocks.len() {
        for support in blocks[i].supported_by.clone() {
            blocks[support].supports.insert(i);
        }
    }

    // Fill out the rest of the paths…
    let mut paths = vec![];
    for (i, block) in blocks.iter().enumerate() {
        let mut path = HashSet::from([i]);
        let mut curr = VecDeque::from_iter(block.supports.iter().cloned());
        while let Some(support) = curr.pop_front() {
            let block = &blocks[support];
            if path.is_superset(&block.supported_by) {
                path.insert(support);
                curr.extend(block.supports.clone());
            }
        }
        path.remove(&i);
        paths.push(path);
    }
    for (i, path) in paths.into_iter().enumerate() {
        blocks[i].supports = path;
    }
}

fn process_data_a(data: &str) -> usize {
    let mut blocks = parse(data);

    drop_blocks(&mut blocks);

    // find all the blocks that aren't supporting anything.
    blocks
        .iter()
        .filter(|&block| block.supports.is_empty())
        .count()
}

fn process_data_b(data: &str) -> usize {
    let mut blocks = parse(data);

    drop_blocks(&mut blocks);

    let answer: usize = blocks.iter().map(|block| block.supports.len()).sum();
    answer
}

//-----------------------------------------------------
// Questions.

q_impl!("22");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9
    "
        )),
        5
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9
    "
        )),
        7
    );
}
