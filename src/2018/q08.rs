//-----------------------------------------------------
// Setup.

static INPUT : &'static str = include_str!("data/q08.data");

fn process_data_a(_data: &str) -> i32 {
    0
}

fn process_data_b(_data: &str) -> i32 {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("08");

#[test]
fn a() {
    assert_eq!(process_data_a("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
