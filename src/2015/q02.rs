//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q02.data");

fn get_pieces(line: &str) -> Vec<u32> {
    let mut rv: Vec<u32> = line.split('x').map(|i| i.parse().unwrap()).collect();
    rv.sort_unstable();
    rv
}

fn process_data_a(data: &str) -> u32 {
    let mut rv = 0;
    for line in data.lines() {
        let pieces = get_pieces(line);
        let size =
            3 * pieces[0] * pieces[1] + 2 * pieces[0] * pieces[2] + 2 * pieces[1] * pieces[2];
        // println!("{:?} => {}", pieces, size);
        rv += size;
    }
    rv
}

fn process_data_b(data: &str) -> u32 {
    let mut rv = 0;
    for line in data.lines() {
        let pieces = get_pieces(line);
        let size = 2 * pieces[0] + 2 * pieces[1] + pieces[0] * pieces[1] * pieces[2];
        // println!("{:?} => {}", pieces, size);
        rv += size;
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("2");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("2x3x4"), 58);
    assert_eq!(process_data_a("1x1x10"), 43);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("2x3x4"), 34);
    assert_eq!(process_data_b("1x1x10"), 14);
}
