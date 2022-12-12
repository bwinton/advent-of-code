//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q08.data");

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    for line in data.lines() {
        rv += 2;
        let mut rest = line[1..line.len() - 1].chars();
        while let Some(curr) = rest.next() {
            if curr != '\\' {
                continue;
            }
            let next = rest.next();
            match next {
                Some('\\') => rv += 1,
                Some('\"') => rv += 1,
                Some('x') => {
                    rv += 3;
                    rest.next();
                    rest.next();
                }
                _ => {
                    println!("Got \\{:?}", next);
                    panic!("Unknown escape sequence!!!")
                }
            }
        }
    }
    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut rv = 0;
    for line in data.lines() {
        rv += 2;
        let rest = line.chars();
        for curr in rest {
            match curr {
                '\\' => rv += 1,
                '"' => rv += 1,
                _ => {}
            }
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("8");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("\"\""), 2);
    assert_eq!(process_data_a("\"abc\""), 2);
    assert_eq!(process_data_a("\"aaa\\\"aaa\""), 3);
    assert_eq!(process_data_a("\"\\x27\""), 5);
    assert_eq!(
        process_data_a(
            "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"",
        ),
        12
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("\"\""), 4);
    assert_eq!(process_data_b("\"abc\""), 4);
    assert_eq!(process_data_b("\"aaa\\\"aaa\""), 6);
    assert_eq!(process_data_b("\"\\x27\""), 5);
    assert_eq!(
        process_data_b(
            "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"",
        ),
        19
    );
}
