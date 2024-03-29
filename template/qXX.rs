//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/qXX.data");

fn process_data_a(data: &str) -> usize {
    let rv = 0;
    for _line in data.lines() {
        // Do something
    }
    rv
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

q_impl!("X");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("")), 0);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
