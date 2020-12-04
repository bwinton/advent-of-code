//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q04.data");

fn process_data_a(data: &str) -> usize {
    for line in data.lines() {
        // Do something
    }
    let mut rv = 0;
    rv
}

fn process_data_b(data: &str) -> usize {
    for line in data.lines() {
        // Do something
    }
    let mut rv = 0;
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
