//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q25.data");

fn from_snafu(input: &str) -> i64 {
    let mut rv = 0;
    for c in input.chars() {
        rv *= 5;
        match c {
            '2' => rv += 2,
            '1' => rv += 1,
            '0' => {}
            '-' => rv -= 1,
            '=' => rv -= 2,
            _ => panic!("Unknown character {}", c),
        }
    }
    rv
}

fn to_snafu(input: i64) -> String {
    let mut input = input;
    let mut rv = vec![];
    while input > 0 {
        match input % 5 {
            4 => {
                rv.push('-');
                input += 5
            }
            3 => {
                rv.push('=');
                input += 5
            }
            2 => rv.push('2'),
            1 => rv.push('1'),
            0 => rv.push('0'),
            _ => panic!("Can't get {} as a remainder!", input % 5),
        }
        input /= 5;
    }
    rv.reverse();
    String::from_iter(&rv)
}

fn process_data_a(data: &str) -> String {
    let rv: i64 = data.lines().map(from_snafu).sum();
    to_snafu(rv)
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

    assert_eq!(from_snafu("1"), 1);
    assert_eq!(from_snafu("2"), 2);
    assert_eq!(from_snafu("1="), 3);
    assert_eq!(from_snafu("1-"), 4);
    assert_eq!(from_snafu("10"), 5);
    assert_eq!(from_snafu("11"), 6);
    assert_eq!(from_snafu("12"), 7);
    assert_eq!(from_snafu("2="), 8);
    assert_eq!(from_snafu("2-"), 9);
    assert_eq!(from_snafu("20"), 10);
    assert_eq!(from_snafu("1=0"), 15);
    assert_eq!(from_snafu("1-0"), 20);
    assert_eq!(from_snafu("1=11-2"), 2022);
    assert_eq!(from_snafu("1-0---0"), 12345);
    assert_eq!(from_snafu("1121-1110-1=0"), 314159265);

    assert_eq!(to_snafu(1), "1");
    assert_eq!(to_snafu(2), "2");
    assert_eq!(to_snafu(3), "1=");
    assert_eq!(to_snafu(4), "1-");
    assert_eq!(to_snafu(5), "10");
    assert_eq!(to_snafu(6), "11");
    assert_eq!(to_snafu(7), "12");
    assert_eq!(to_snafu(8), "2=");
    assert_eq!(to_snafu(9), "2-");
    assert_eq!(to_snafu(10), "20");
    assert_eq!(to_snafu(15), "1=0");
    assert_eq!(to_snafu(20), "1-0");
    assert_eq!(to_snafu(2022), "1=11-2");
    assert_eq!(to_snafu(12345), "1-0---0");
    assert_eq!(to_snafu(314159265), "1121-1110-1=0");

    assert_eq!(
        process_data_a(indoc!(
            "1=-0-2
    12111
    2=0=
    21
    2=01
    111
    20012
    112
    1=-1=
    1-12
    12
    1=
    122
    "
        )),
        "2=-1=0"
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
