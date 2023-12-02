use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};
// use itertools::Itertools;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q12.data");

fn process_data_a(data: &str) -> usize {
    let mut rooms: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in data.lines() {
        // Do something
        let (start, end) = line.split_once('-').unwrap();
        rooms.entry(start).or_default().insert(end);
        rooms.entry(end).or_default().insert(start);
    }

    let mut paths = 0;
    let mut stack = vec![];
    let mut seen: HashSet<Vec<&str>> = HashSet::new();
    stack.push(vec!["start"]);
    while let Some(curr_path) = stack.pop() {
        let &curr_room = curr_path.last().unwrap();
        for &next in &rooms[curr_room] {
            if next == "end" {
                // Found one!
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

const START_LOCATION: usize = 0;
const END_LOCATION: usize = 1;

fn get_rooms(data: &str) -> (Vec<HashSet<usize>>, HashSet<usize>) {
    let max_rooms = data.lines().count();
    let mut room_names = vec!["start", "end"];
    let mut small_rooms = HashSet::with_capacity(50);
    let mut rooms = vec![HashSet::new(); max_rooms];
    for line in data.lines() {
        // Do something
        let (start_name, end_name) = line.split_once('-').unwrap();
        let start = room_names
            .iter()
            .position(|&x| x == start_name)
            .unwrap_or_else(|| {
                room_names.push(start_name);
                room_names.len() - 1
            });
        let end = room_names
            .iter()
            .position(|&x| x == end_name)
            .unwrap_or_else(|| {
                room_names.push(end_name);
                room_names.len() - 1
            });

        if !small_rooms.contains(&start) && start_name.chars().all(|c| c.is_lowercase()) {
            small_rooms.insert(start);
        }
        if !small_rooms.contains(&end) && end_name.chars().all(|c| c.is_lowercase()) {
            small_rooms.insert(end);
        }

        if start_name != "end" && end_name != "start" {
            rooms[start].insert(end);
        }
        if start_name != "start" && end_name != "end" {
            rooms[end].insert(start);
        }
    }
    (rooms, small_rooms)
}

fn get_paths(rooms: Vec<HashSet<usize>>, small_rooms: HashSet<usize>) -> usize {
    let mut paths = 0;
    let mut stack = Vec::with_capacity(50);
    let mut path = Vec::with_capacity(50);
    path.push(START_LOCATION);
    stack.push((path, START_LOCATION, false));
    while let Some((curr_path, curr_room, found_twice)) = stack.pop() {
        for &next in &rooms[curr_room] {
            if next == END_LOCATION {
                // Found one!
                paths += 1;
                continue;
            }
            let mut next_path = curr_path.clone();
            let mut found_twice = found_twice;
            next_path.push(next);

            if small_rooms.contains(&next) {
                match next_path.iter().filter(|&v| v == &next).count().cmp(&2) {
                    Ordering::Greater => {
                        continue;
                    }
                    Ordering::Equal => {
                        if found_twice {
                            continue;
                        } else {
                            found_twice = true;
                        }
                    }
                    Ordering::Less => {}
                }
            }
            stack.push((next_path, next, found_twice));
        }
    }
    paths
}

fn process_data_b(data: &str) -> usize {
    let (rooms, small_rooms) = get_rooms(data);
    get_paths(rooms, small_rooms)
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

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
    use pretty_assertions::assert_eq;

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
