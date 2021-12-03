//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q02.data");

fn process_data_a(data: &str) -> i32 {
    let mut position = (0, 0);
    for line in data.lines() {
        let (direction, value) = line.split_once(' ').unwrap();
        let value: i32 = value.parse().unwrap();
        match direction {
            "forward" => position.0 += value,
            "down" => position.1 += value,
            "up" => position.1 -= value,
            _ => {
                println!("Unknown direction: {:?}", direction)
            }
        }
        // Do something
    }
    position.0 * position.1
}

fn process_data_b(data: &str) -> i32 {
    let mut position = (0, 0);
    let mut aim = 0;
    for line in data.lines() {
        let (direction, value) = line.split_once(' ').unwrap();
        let value: i32 = value.parse().unwrap();
        match direction {
            "forward" => {
                position.0 += value;
                position.1 += value * aim
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => {
                println!("Unknown direction: {:?}", direction)
            }
        }
        // Do something
    }
    position.0 * position.1
}

//-----------------------------------------------------
// Questions.

q_impl!("2");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2"
        )),
        150
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2"
        )),
        900
    );
}
