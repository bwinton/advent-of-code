//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/qXX.data");

fn process_data_a(_data: &str) -> usize {
    0
}

fn process_data_b(_data: &str) -> usize {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("XX");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
