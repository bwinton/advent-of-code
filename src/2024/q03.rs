use regex::Regex;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q03.data");

fn process_data_a(data: &str) -> i32 {
    let mut rv = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for line in data.lines() {
        for c in re.captures_iter(line) {
            let (_full, [first, second]) = c.extract();
            let first: i32 = first.parse().unwrap();
            let second: i32 = second.parse().unwrap();
            rv += first * second;
        }
    }
    rv
}

fn process_data_b(data: &str) -> i32 {
    let mut rv = 0;
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut add = true;
    for line in data.lines() {
        for c in re.captures_iter(line) {
            let full = c.get(0).unwrap().as_str();
            match &full[0..4] {
                "do()" => add = true,
                "don'" => add = false,
                "mul(" => {
                    if add {
                        let first: i32 = c.get(1).unwrap().as_str().parse().unwrap();
                        let second: i32 = c.get(2).unwrap().as_str().parse().unwrap();
                        rv += first * second;
                    }
                }
                x => panic!("Don't understand {}", x),
            }
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        )),
        161
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
        )),
        48
    );
}
