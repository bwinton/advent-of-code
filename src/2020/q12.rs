//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q12.data");

fn turn(waypoint: (isize, isize), angle: isize) -> (isize, isize) {
    match angle {
        90 => (waypoint.1, -waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (-waypoint.1, waypoint.0),
        _ => {
            panic!("Invalid heading! {}", angle);
        }
    }
}

fn process_data_a(data: &str) -> isize {
    let mut instructions = vec![];
    for line in data.lines() {
        let mut chars = line.chars();
        let inst = chars.next().unwrap();
        let number: isize = chars.collect::<String>().parse().unwrap();
        instructions.push((inst, number));
    }
    let mut position = (0, 0);
    let mut heading = (1, 0);
    for (instruction, number) in instructions {
        match instruction {
            'N' => position.1 -= number,
            'E' => position.0 += number,
            'S' => position.1 += number,
            'W' => position.0 -= number,
            'L' => heading = turn(heading, number),
            'R' => heading = turn(heading, 360 - number),
            'F' => {
                position.0 += heading.0 * number;
                position.1 += heading.1 * number;
            }
            _ => {
                panic!("Unknown instruction {}", instruction);
            }
        }
    }
    position.0 + position.1
}

fn process_data_b(data: &str) -> isize {
    let mut instructions = vec![];
    for line in data.lines() {
        let mut chars = line.chars();
        let inst = chars.next().unwrap();
        let number: isize = chars.collect::<String>().parse().unwrap();
        instructions.push((inst, number));
    }
    let mut position = (0, 0);
    let mut waypoint = (10, -1);
    for (instruction, number) in instructions {
        match instruction {
            'N' => waypoint.1 -= number,
            'E' => waypoint.0 += number,
            'S' => waypoint.1 += number,
            'W' => waypoint.0 -= number,
            'L' => waypoint = turn(waypoint, number),
            'R' => waypoint = turn(waypoint, 360 - number),
            'F' => {
                position.0 += waypoint.0 * number;
                position.1 += waypoint.1 * number;
            }
            _ => {
                panic!("Unknown instruction {}", instruction);
            }
        }
    }
    position.0.abs() + position.1.abs()
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "F10
N3
F7
R90
F11"
        ),
        25
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "F10
N3
F7
R90
F11"
        ),
        286
    );
}
