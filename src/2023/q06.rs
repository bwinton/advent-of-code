//-----------------------------------------------------
// Setup.

use itertools::Itertools;

static INPUT: &str = include_str!("data/q06.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 1;
    let mut lines = data.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let races = times
        .into_iter()
        .zip(distances)
        .collect::<Vec<(usize, usize)>>();
    for (time, target) in races {
        let mut wins = 0;
        for i in 0..=time / 2 {
            if i * (time - i) > target {
                if i != time - i {
                    wins += 2;
                } else {
                    wins += 1;
                }
            }
        }
        rv *= wins;
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut lines = data.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time: ")
        .unwrap()
        .split_ascii_whitespace()
        .join("")
        .parse::<usize>()
        .unwrap();
    let target = lines
        .next()
        .unwrap()
        .strip_prefix("Distance: ")
        .unwrap()
        .split_ascii_whitespace()
        .join("")
        .parse::<usize>()
        .unwrap();
    let mut wins = 0;
    for i in 0..=time / 2 {
        if i * (time - i) > target {
            if i != time - i {
                wins += 2;
            } else {
                wins += 1;
            }
        }
    }
    wins
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "Time:      7  15   30
    Distance:  9  40  200
    "
        )),
        288
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "Time:      7  15   30
    Distance:  9  40  200
    "
        )),
        71503
    );
}
