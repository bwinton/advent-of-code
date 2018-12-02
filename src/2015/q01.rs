//-----------------------------------------------------
// Setup.

static INPUT: &'static str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    for character in data.chars() {
        match character {
            '(' => rv += 1,
            ')' => rv -= 1,
            _ => panic!("Invalid Character \"{}\"", character),
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    for (i, character) in data.chars().enumerate() {
        match character {
            '(' => rv += 1,
            ')' => rv -= 1,
            _ => panic!("Invalid Character \"{}\"", character),
        }
        if rv < 0 {
            return i + 1;
        }
    }
    unreachable!()
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn test_a() {
    assert_eq!(process_data_a("(())"), 0);
    assert_eq!(process_data_a("()()"), 0);
    assert_eq!(process_data_a("((("), 3);
    assert_eq!(process_data_a("(()(()("), 3);
    assert_eq!(process_data_a("))((((("), 3);
    assert_eq!(process_data_a("())"), -1);
    assert_eq!(process_data_a("))("), -1);
    assert_eq!(process_data_a(")())())"), -3);
}

#[test]
fn test_b() {
    assert_eq!(process_data_b(")"), 1);
    assert_eq!(process_data_b("()())"), 5);
}
