//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q09.data");

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    for line in data.lines() {
        let mut values: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut last_values = vec![*values.last().unwrap()];
        while !values.iter().all(|&value| value == 0) {
            let mut next = vec![];
            for (i, value) in values.iter().enumerate() {
                if i == values.len() - 1 {
                    continue;
                }
                next.push(values[i + 1] - value);
            }
            values = next;
            last_values.push(*values.last().unwrap());
        }
        last_values.reverse();
        let mut curr = 0;
        for value in last_values {
            curr += value;
        }
        rv += curr;
    }
    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut rv = 0;
    for line in data.lines() {
        let mut values: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut first_values = vec![*values.first().unwrap()];
        while !values.iter().all(|&value| value == 0) {
            let mut next = vec![];
            for (i, value) in values.iter().enumerate() {
                if i == values.len() - 1 {
                    continue;
                }
                next.push(values[i + 1] - value);
            }
            values = next;
            first_values.push(*values.first().unwrap());
        }
        first_values.reverse();
        let mut curr = 0;
        for value in first_values {
            curr = value - curr;
        }
        rv += curr;
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
            "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    "
        )),
        114
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    "
        )),
        2
    );
}
