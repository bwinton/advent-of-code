//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet, VecDeque};

static INPUT: &str = include_str!("data/q25.data");

fn parse(data: &str) -> HashMap<&str, HashSet<&str>> {
    let mut rv: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in data.lines() {
        let (component, list) = line.split_once(": ").unwrap();
        let components: HashSet<_> = list.split_ascii_whitespace().collect();
        for backwards in &components {
            rv.entry(*backwards).or_default().insert(component);
        }
        rv.entry(component).or_default().extend(components);
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mapping = parse(data);
    let first = mapping.keys().cloned().next().unwrap();
    let mut near_group = vec![];
    let mut far_group = vec![first];
    for target in mapping.clone().into_keys() {
        if target == first {
            continue;
        }
        let mut connections = 0;

        let mut seen = HashSet::from([first]);
        for &component in mapping.get(first).unwrap() {
            if component == target {
                connections += 1;
                continue;
            }
            let mut upcoming = HashSet::new();
            let mut states = VecDeque::new();
            states.push_back((component, vec![component]));
            let mut found = false;
            while !states.is_empty() && !found && connections < 4 {
                let (curr, path) = states.pop_front().unwrap();
                for &next in mapping.get(curr).unwrap() {
                    if next == target {
                        connections += 1;
                        seen.extend(path);
                        found = true;
                        break;
                    } else if !upcoming.contains(next)
                        && !path.contains(&next)
                        && !seen.contains(next)
                    {
                        let mut next_path = path.clone();
                        next_path.push(next);
                        states.push_back((next, next_path));
                        upcoming.insert(next);
                    }
                }
            }
        }
        if connections >= 4 {
            far_group.push(target);
        } else {
            near_group.push(target);
        }
    }

    near_group.len() * far_group.len()
}

fn process_data_b(data: &str) -> usize {
    let rv = 0;
    for _line in data.lines() {
        // Do something
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("25");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr
    "
        )),
        54
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
