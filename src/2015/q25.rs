//-----------------------------------------------------
// Setup.

static INPUT: &'static str = "";

fn find_cell(row: usize, column: usize) -> usize {
    (1..)
        .take(row - 1)
        .chain((1..).skip(row).take(column - 1))
        .sum()
}

fn step(rv: i64) -> i64 {
    (rv * 252_533) % 33_554_393
}

fn process_data_a_impl(row: usize, column: usize) -> i64 {
    let mut rv = 20_151_125;
    for _ in 0..find_cell(row, column) {
        rv = step(rv);
    }
    rv
}

fn process_data_a(_data: &str) -> i64 {
    process_data_a_impl(2978, 3083)
}

fn process_data_b(_data: &str) -> i32 {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("25");

#[test]
fn a() {
    assert_eq!(find_cell(1, 1), 0);
    assert_eq!(find_cell(1, 6), 20);
    assert_eq!(find_cell(6, 1), 15);
    assert_eq!(find_cell(3, 2), 7);
    assert_eq!(find_cell(4, 2), 11);
    assert_eq!(find_cell(1, 5), 14);
    assert_eq!(find_cell(6, 6), 60);
    assert_eq!(find_cell(2978, 3083), 18_361_852);
    assert_eq!(process_data_a_impl(1, 1), 20_151_125);
    assert_eq!(process_data_a_impl(2, 1), 31_916_031);
    assert_eq!(process_data_a_impl(1, 2), 18_749_137);
    assert_eq!(process_data_a_impl(6, 6), 27_995_004);
    assert_eq!(process_data_a_impl(2978, 3083), 2_650_453);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
