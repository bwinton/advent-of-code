//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q20.data");

fn process(data: &str, key: i64, reps: usize) -> i64 {
    let mut values: Vec<(usize, i64)> = data
        .lines()
        .enumerate()
        .map(|(i, x)| (i, x.parse::<i64>().unwrap() * key))
        .collect();
    let order = values.clone();
    for _ in 0..reps {
        for &(i, next) in &order {
            let index = values.iter().position(|&x| x == (i, next)).unwrap();

            values.remove(index);
            let end = (index as i64 + next).rem_euclid(values.len() as i64) as usize;
            values.insert(end, (i, next));
        }
    }
    let index = values.iter().position(|&(_, x)| x == 0).unwrap();
    let first = (index + 1000) % values.len();
    let second = (index + 2000) % values.len();
    let third = (index + 3000) % values.len();
    values[first].1 + values[second].1 + values[third].1
}

fn process_data_a(data: &str) -> i64 {
    process(data, 1, 1)
}

fn process_data_b(data: &str) -> i64 {
    process(data, 811_589_153, 10)
}

//-----------------------------------------------------
// Questions.

q_impl!("20");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "1
    2
    -3
    3
    -2
    0
    4
    "
        )),
        3
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "1
    2
    -3
    3
    -2
    0
    4
    "
        )),
        1_623_178_306
    );
}
