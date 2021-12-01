//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let mut prev = 9999999;
    for line in data.lines() {
        let curr: u32 = line.parse().unwrap();
        if curr > prev {
            rv += 1;
        }
        prev = curr;
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let mut lines = data.lines();
    let mut a: i32 = lines.next().unwrap().parse().unwrap();
    let mut b: i32 = lines.next().unwrap().parse().unwrap();
    let mut c: i32 = lines.next().unwrap().parse().unwrap();
    for line in lines {
        let d = line.parse().unwrap();
        if d > a {
            rv += 1;
        };
        a = b;
        b = c;
        c = d;
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "199
    200
    208
    210
    200
    207
    240
    269
    260
    263"
        )),
        7
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "199
    200
    208
    210
    200
    207
    240
    269
    260
    263"
        )),
        5
    );
}
