use std::collections::HashSet;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q25.data");

type Positions = HashSet<(usize, usize)>;

fn step(easts: &mut Positions, souths: &mut Positions, max_x: usize, max_y: usize) -> bool {
    let mut moved = false;

    // Move the easts east.
    let mut next = HashSet::new();
    for &(x, y) in easts.iter() {
        let x1 = (x + 1) % max_x;
        if easts.contains(&(x1, y)) || souths.contains(&(x1, y)) {
            next.insert((x, y));
        } else {
            next.insert((x1, y));
            moved = true;
        }
    }
    *easts = next;

    // Move the souths south.
    let mut next = HashSet::new();
    for &(x, y) in souths.iter() {
        let y1 = (y + 1) % max_y;
        if easts.contains(&(x, y1)) || souths.contains(&(x, y1)) {
            next.insert((x, y));
        } else {
            next.insert((x, y1));
            moved = true;
        }
    }
    *souths = next;

    moved
}

fn parse_data(data: &str) -> (Positions, Positions, usize, usize) {
    let mut easts = HashSet::new();
    let mut souths = HashSet::new();
    let mut max_x = 0;
    let max_y = data.lines().count();
    for (y, line) in data.lines().enumerate() {
        if max_x == 0 {
            max_x = line.chars().count();
        }
        for (x, char) in line.chars().enumerate() {
            match char {
                '>' => {
                    easts.insert((x, y));
                }
                'v' => {
                    souths.insert((x, y));
                }
                '.' => {}
                _ => panic!("Unknnown character {}!", char),
            };
        }
    }
    (easts, souths, max_x, max_y)
}

#[allow(dead_code)]
fn print_board(easts: &Positions, souths: &Positions, max_x: usize, max_y: usize) {
    for y in 0..max_y {
        for x in 0..max_x {
            match (easts.contains(&(x, y)), souths.contains(&(x, y))) {
                (true, false) => {
                    print!(">");
                }
                (false, true) => {
                    print!("v");
                }
                (false, false) => {
                    print!(".");
                }
                (true, true) => {
                    print!("?");
                }
            }
        }
        println!();
    }
}

fn process_data_a(data: &str) -> usize {
    let (mut easts, mut souths, max_x, max_y) = parse_data(data);
    let mut i = 0;
    loop {
        // print_board(&easts, &souths, max_x, max_y);
        // println!("{}, {}, {:?}, {:?}\n", max_x, max_y, &easts, &souths);

        let moved = step(&mut easts, &mut souths, max_x, max_y);
        // println!("{}, {}", i, moved);
        if !moved {
            break;
        }
        i += 1;
    }
    i + 1
}

fn process_data_b(data: &str) -> usize {
    data.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("25");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "v...>>.vv>
    .vv>>.vv..
    >>.>v>...v
    >>v>>.>.v.
    v>v.vv.v..
    >.>>..v...
    .vv..>.>v.
    v.v..>>v.v
    ....v..v.>
    "
        )),
        58
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
