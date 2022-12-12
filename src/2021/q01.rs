//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q01.data");

fn body(data: &[i32], skip: usize) -> usize {
    data.iter()
        .zip(data.iter().skip(skip))
        .filter(|(a, b)| a < b)
        .count()
}

fn process_data_a(data: &str) -> usize {
    body(
        &data
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>(),
        1,
    )
}

fn process_data_b(data: &str) -> usize {
    body(
        &data
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>(),
        3,
    )
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "199
    200
    208
    210
    200
    207
    240
    269
    260
    263"
        )),
        7
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "199
    200
    208
    210
    200
    207
    240
    269
    260
    263"
        )),
        5
    );
}
