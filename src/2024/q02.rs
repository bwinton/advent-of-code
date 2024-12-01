//-----------------------------------------------------
// Setup.

use itertools::Itertools;

static INPUT: &str = include_str!("data/q02.data");

fn get_level_error(numbers: &[i32]) -> Option<usize> {
    let mut direction = None;
    let mut valid = None;
    for (i, (a, b)) in numbers.iter().tuple_windows().enumerate() {
        if direction.is_none() {
            direction = Some(a < b);
        }
        let difference = (a - b).abs();
        if difference == 0 || difference > 3 || direction != Some(a < b) {
            valid = Some(i);
            break;
        }
    }
    valid
}

fn process_data_a(data: &str) -> usize {
    let mut rv: usize = 0;
    for line in data.lines() {
        let numbers = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if get_level_error(&numbers).is_none() {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv: usize = 0;
    for line in data.lines() {
        let numbers = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if let Some(index) = get_level_error(&numbers) {
            // try removing the three around the failure.
            if index >= 1 {
                let mut test = numbers.clone();
                test.remove(index - 1);
                if get_level_error(&test).is_none() {
                    rv += 1;
                    continue;
                }
            }

            let mut test = numbers.clone();
            test.remove(index);
            if get_level_error(&test).is_none() {
                rv += 1;
                continue;
            }

            if index < numbers.len() - 1 {
                let mut test = numbers.clone();
                test.remove(index + 1);
                if get_level_error(&test).is_none() {
                    rv += 1;
                    continue;
                }
            }
        } else {
            rv += 1;
        }
    }
    rv
    // 484 is too low
}

//-----------------------------------------------------
// Questions.

q_impl!("2");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
        )),
        2
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
        )),
        4
    );

    assert_eq!(process_data_b(indoc!("10 9 10 11")), 1);
}
