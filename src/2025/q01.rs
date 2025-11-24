//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let mut dial = 50;
    for line in data.lines() {
        let (turn, degrees) = line.split_at(1);
        let mut degrees: usize = degrees.parse().unwrap();
        if turn == "L" {
            degrees = 100 - degrees % 100;
        }
        dial += degrees;
        dial %= 100;
        if dial == 0 {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let mut dial = 50;
    for line in data.lines() {
        let (turn, degrees) = line.split_at(1);
        let degrees: usize = degrees.parse().unwrap();
        if turn == "R" {
            dial += degrees;
            rv += dial / 100;
            dial %= 100;
        } else {
            if degrees > 100 {
                rv += degrees / 100;
            }
            let degrees = degrees % 100;
            if degrees > dial {
                if dial != 0 {
                    rv += 1;
                }
                dial += 100;
            }
            dial -= degrees;

            rv += dial / 100;
            dial %= 100;
            if dial == 0 && degrees != 0 {
                rv += 1;
            }
        }
    }

    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
        )),
        3
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("R1000")), 10);
    assert_eq!(process_data_b(indoc!("L249")), 2);
    assert_eq!(process_data_b(indoc!("L250")), 3);
    assert_eq!(process_data_b(indoc!("L251")), 3);
    assert_eq!(
        process_data_b(indoc!(
            "R50
    L200
    "
        )),
        3
    );

    assert_eq!(
        process_data_b(indoc!(
            "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82"
        )),
        6
    );
}
