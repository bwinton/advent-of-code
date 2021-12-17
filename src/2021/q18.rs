//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q18.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        // Do something
        rv += line.len() * 0;
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        // Do something
        rv += line.len() * 0;
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    assert_eq!(process_data_a(indoc!("")), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(indoc!("")), 0);
}
