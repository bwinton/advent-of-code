//-----------------------------------------------------
// Setup.

use itertools::Itertools;

static INPUT: &str = include_str!("data/q09.data");

fn process_data_a(data: &str) -> i64 {
    let mut rv = 0;
    let mut tiles = vec![];
    for line in data.lines() {
        let pos: Vec<i64> = line.split(",").map(|c| c.parse().unwrap()).collect();
        tiles.push((pos[0], pos[1]));
    }
    for (a, b) in tiles.into_iter().tuple_combinations() {
        let width = (a.0 - b.0).abs() + 1;
        let height = (a.1 - b.1).abs() + 1;
        let size = width * height;
        if size > rv {
            rv = size;
        }
    }
    rv
}

fn process_data_b(data: &str) -> i64 {
    let mut rv = 0;
    let mut tiles = vec![];
    for line in data.lines() {
        let pos: Vec<i64> = line.split(",").map(|c| c.parse().unwrap()).collect();
        tiles.push((pos[0], pos[1]));
    }
    let mut edges = vec![];
    for (a, b) in tiles.iter().circular_tuple_windows() {
        edges.push((*a, *b));
    }
    for (a, b) in tiles.iter().tuple_combinations() {
        let external = edges.iter().any(|(start, end)| {
            if start == a || start == b || end == a || end == b {
                return false;
            }
            let edge_min_x = start.0.min(end.0);
            let edge_max_x = start.0.max(end.0);
            let edge_min_y = start.1.min(end.1);
            let edge_max_y = start.1.max(end.1);

            let box_min_x = a.0.min(b.0);
            let box_max_x = a.0.max(b.0);
            let box_min_y = a.1.min(b.1);
            let box_max_y = a.1.max(b.1);

            let left = edge_max_x <= box_min_x;
            let right = edge_min_x >= box_max_x;
            let up = edge_max_y <= box_min_y;
            let down = edge_min_y >= box_max_y;
            !(left || right || up || down)
        });
        if external {
            continue;
        }
        let width = (a.0 - b.0).abs() + 1;
        let height = (a.1 - b.1).abs() + 1;
        let size = width * height;
        if size > rv {
            rv = size;
        }
    }

    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("9");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        )),
        50
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        )),
        24
    );
}
