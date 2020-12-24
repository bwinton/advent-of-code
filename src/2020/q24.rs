//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q24.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        // Do something
        rv += line.len();
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        // Do something
        rv += line.len();
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("24");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
