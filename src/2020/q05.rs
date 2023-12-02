//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q05.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let line = line
            .replace('F', "0")
            .replace(['B', 'R'], "1")
            .replace('L', "0");
        let id = usize::from_str_radix(&line, 2).unwrap();
        if id > rv {
            rv = id;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = vec![];
    for line in data.lines() {
        let line = line
            .replace('F', "0")
            .replace(['B', 'R'], "1")
            .replace('L', "0");
        rv.push(usize::from_str_radix(&line, 2).unwrap());
    }
    rv.sort_unstable();
    for (index, &item) in rv.iter().enumerate().skip(1) {
        if item - 1 != rv[index - 1] {
            return item - 1;
        }
    }
    rv.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    // f=0 b=1 r=1 l=0
    // FBFBBFFRLR: row 44, column 5, seat ID 357.
    assert_eq!(process_data_a("FBFBBFFRLR"), 357);
    assert_eq!(process_data_a("BFFFBBFRRR"), 567);
    assert_eq!(process_data_a("FFFBBBFRRR"), 119);
    assert_eq!(process_data_a("BBFFBBFRLL"), 820);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(""), 0);
}
