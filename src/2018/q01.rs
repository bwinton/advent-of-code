//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> i64 {
    data.lines().map(|x| x.parse::<i64>().unwrap()).sum()
}

fn process_data_b(data: &str) -> i64 {
    let mut curr = 0;
    let mut values = data.lines().map(|x| x.parse::<i64>().unwrap()).cycle();
    let mut seen = HashSet::new();

    while seen.insert(curr) {
        curr += values.next().unwrap();
    }
    curr
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "+1
-2
+3
+1"
        ),
        3
    );
    assert_eq!(
        process_data_a(
            "+1
+1
+1"
        ),
        3
    );
    assert_eq!(
        process_data_a(
            "+1
+1
-2"
        ),
        0
    );
    assert_eq!(
        process_data_a(
            "-1
-2
-3"
        ),
        -6
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "+1
-2
+3
+1"
        ),
        2
    );
    assert_eq!(
        process_data_b(
            "+1
-1"
        ),
        0
    );
    assert_eq!(
        process_data_b(
            "+3
+3
+4
-2
-4"
        ),
        10
    );
    assert_eq!(
        process_data_b(
            "-6
+3
+8
+5
-6"
        ),
        5
    );
    assert_eq!(
        process_data_b(
            "+7
+7
-2
-7
-4"
        ),
        14
    );
}
