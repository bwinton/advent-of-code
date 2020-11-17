//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(_data: &str) -> i32 {
    0
}

fn process_data_b(_data: &str) -> i32 {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
