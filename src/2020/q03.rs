//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q03.data");

fn process(data: &str, values: &[(usize, usize)]) -> usize {
    let mut map: Vec<Vec<bool>> = vec![];
    for line in data.lines() {
        // Do something
        let mut curr = vec![];
        for cell in line.chars() {
            curr.push(cell == '#');
        }
        map.push(curr);
    }
    let mut rv = 1;
    for slope in values {
        let mut pos = (0, 0);
        let mut curr = 0;
        while pos.1 < map.len() {
            if map[pos.1][pos.0] {
                curr += 1;
            }
            pos.0 = (pos.0 + slope.0) % map[0].len();
            pos.1 += slope.1;
        }
        // println!("{:?}, {}", slope, curr);
        rv *= curr;
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    process(data, &[(3, 1)])
}

fn process_data_b(data: &str) -> usize {
    process(data, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
        ),
        7
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
        ),
        336
    );
}
