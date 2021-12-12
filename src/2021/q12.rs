use std::collections::{HashMap, HashSet};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q12.data");

fn process_data_a(data: &str) -> usize {
    let mut rooms: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in data.lines() {
        // Do something
        let (start, end) = line.split_once("-").unwrap();
        rooms.entry(start).or_default().insert(end);
        rooms.entry(end).or_default().insert(start);
    }

    // println!("{:?}", rooms);

    let mut paths = 0;
    let mut stack = vec![];
    let mut seen: HashSet<Vec<&str>> = HashSet::new();
    stack.push(vec!["start"]);
    while !stack.is_empty() {
        let curr_path = stack.pop().unwrap();
        let &curr_room = curr_path.last().unwrap();
        for &next in &rooms[curr_room] {
            if next == "end" {
                // Found one!
                // println!("{:?}->{}", curr_path.join("->"), next);
                paths += 1;
                continue;
            }
            if next.chars().all(|c| c.is_lowercase()) && curr_path.contains(&next) {
                continue;
            }
            let mut next_path = curr_path.clone();
            next_path.push(next);
            if !seen.contains(&next_path) && !stack.contains(&next_path) {
                stack.push(next_path.clone());
                seen.insert(next_path);
            }
        }
    }
    paths
}

fn process_data_b(data: &str) -> usize {
    let mut rooms: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in data.lines() {
        // Do something
        let (start, end) = line.split_once("-").unwrap();
        if start != "end" && end != "start" {
            rooms.entry(start).or_default().insert(end);
        }
        if start != "start" && end != "end" {
            rooms.entry(end).or_default().insert(start);
        }
    }

    // println!("{:?}", rooms);

    let mut paths = 0;
    let mut stack = Vec::with_capacity(50);
    let mut seen: HashSet<Vec<&str>> = HashSet::with_capacity(rooms.len() + 10);
    stack.push((vec!["start"], HashMap::new()));
    while !stack.is_empty() {
        let (curr_path, curr_smalls) = stack.pop().unwrap();
        let &curr_room = curr_path.last().unwrap();
        for &next in &rooms[curr_room] {
            if next == "end" {
                // Found one!
                // println!("{:?},{}", curr_path.join(","), next);
                paths += 1;
                continue;
            }
            let mut next_path = curr_path.clone();
            let mut next_smalls: HashMap<&str, usize> = curr_smalls.clone();
            next_path.push(next);
            if next.chars().all(|c| c.is_lowercase()) {
                let test = next_smalls.entry(next).or_default();
                *test += 1;
                // println!("Getting next_smalls: {:?}", next_smalls);
                if *test > 2 || next_smalls.iter().filter(|&(_, &v)| v > 1).count() > 1 {
                    continue;
                }
            }
            if !seen.contains(&next_path) && !stack.iter().any(|(path, _)| path == &next_path) {
                stack.push((next_path.clone(), next_smalls));
                seen.insert(next_path);
            }
        }
    }
    paths
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
    "
        )),
        10
    );
    assert_eq!(
        process_data_a(indoc!(
            "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc
    "
        )),
        19
    );
    assert_eq!(
        process_data_a(indoc!(
            "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW
    "
        )),
        226
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end
    "
        )),
        36
    );
    assert_eq!(
        process_data_b(indoc!(
            "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc
    "
        )),
        103
    );
    assert_eq!(
        process_data_b(indoc!(
            "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW
    "
        )),
        3509
    );
}
