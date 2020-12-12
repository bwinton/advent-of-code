//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q12.data");

#[derive(EnumSetType, Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

fn turn(heading: Heading, direction: char, number: isize) -> Heading {
    match (heading, direction, number) {
        (Heading::North, 'R', 90) => Heading::East,
        (Heading::North, _, 180) => Heading::South,
        (Heading::North, 'L', 90) => Heading::West,

        (Heading::South, 'R', 90) => Heading::West,
        (Heading::South, _, 180) => Heading::North,
        (Heading::South, 'R', 270) => Heading::East,
        (Heading::South, 'L', 90) => Heading::East,

        (Heading::East, 'R', 90) => Heading::South,
        (Heading::East, 'R', 270) => Heading::North,
        (Heading::East, _, 180) => Heading::West,
        (Heading::East, 'L', 90) => Heading::North,

        (Heading::West, 'R', 90) => Heading::North,
        (Heading::West, _, 180) => Heading::East,
        (Heading::West, 'L', 90) => Heading::South,

        _ => {
            panic!("Invalid heading! {:?}, {}, {}", heading, direction, number);
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
    let mut heading = Heading::East;
    for (instruction, number) in instructions {
        match instruction {
            'N' => position.1 -= number,
            'E' => position.0 += number,
            'S' => position.1 += number,
            'W' => position.0 -= number,
            'L' => heading = turn(heading, instruction, number),
            'R' => heading = turn(heading, instruction, number),
            'F' => match heading {
                Heading::North => position.1 -= number,
                Heading::East => position.0 += number,
                Heading::South => position.1 += number,
                Heading::West => position.0 -= number,
            },
            _ => {
                panic!("Unknown instruction {}", instruction);
            }
        }
    }
    position.0 + position.1
}

fn turn_waypoint(waypoint: (isize, isize), direction: char, number: isize) -> (isize, isize) {
    match (direction, number) {
        ('R', 90) => (-waypoint.1, waypoint.0),
        (_, 180) => (-waypoint.0, -waypoint.1),
        ('R', 270) | ('L', 90) => (waypoint.1, -waypoint.0),
        _ => {
            panic!("Invalid heading! {}, {}", direction, number);
        }
    }
}

fn process_data_b(data: &str) -> isize {
    let mut instructions = vec![];
    for line in data.lines() {
        let mut chars = line.chars();
        let inst = chars.next().unwrap();
        let number: isize = chars.collect::<String>().parse().unwrap();
        instructions.push((inst, number));
    }
    // println!("instructions = {:?}", instructions);
    let mut position = (0, 0);
    let mut waypoint = (10, -1);
    for (instruction, number) in instructions {
        // println!("{:?}->{:?} / {}-{}", position, waypoint, instruction, number);
        match instruction {
            'N' => waypoint.1 -= number,
            'E' => waypoint.0 += number,
            'S' => waypoint.1 += number,
            'W' => waypoint.0 -= number,
            'L' => waypoint = turn_waypoint(waypoint, instruction, number),
            'R' => waypoint = turn_waypoint(waypoint, instruction, number),
            'F' => {
                position.0 += waypoint.0 * number;
                position.1 += waypoint.1 * number;
            }
            _ => {
                panic!("Unknown instruction {}", instruction);
            }
        }
    }
    // println!("{:?}->{:?}", position, waypoint);
    position.0.abs() + position.1.abs()
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
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
