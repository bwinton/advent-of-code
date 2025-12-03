//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q03.data");

fn find_highest(line: &[u8], position: usize) -> (usize, u8) {
    let mut rv = 0;
    let mut max = 0;
    let len = line.len();
    for (i, cell) in line.iter().enumerate() {
        let value = cell - b'0';
        if value > max && i <= len - position {
            max = value;
            rv = i;
        }
    }
    (rv, max)
}

fn process_data(data: &str, size: usize) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let mut values = vec![0; size];
        let len = values.len();
        let mut curr = 0;
        let line: Vec<u8> = line.bytes().collect();
        for (i, cell) in values.iter_mut().enumerate() {
            let (index, value) = find_highest(&line[curr..], len - i);
            curr = index + curr + 1;
            *cell = value;
        }
        let mut result = 0;
        for value in values {
            result += value as usize;
            result *= 10;
        }
        rv += result / 10;
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    process_data(data, 2)
}

fn process_data_b(data: &str) -> usize {
    process_data(data, 12)
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("987654321111111")), 98);
    assert_eq!(process_data_a(indoc!("811111111111119")), 89);
    assert_eq!(process_data_a(indoc!("234234234234278")), 78);
    assert_eq!(process_data_a(indoc!("818181911112111")), 92);
    assert_eq!(
        process_data_a(indoc!(
            "987654321111111
            811111111111119
            234234234234278
            818181911112111
            "
        )),
        357
    );
}

#[test]
fn b() {
    // use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("987654321111111")), 987654321111);
    assert_eq!(process_data_b(indoc!("811111111111119")), 811111111119);
    assert_eq!(process_data_b(indoc!("234234234234278")), 434234234278);
    assert_eq!(process_data_b(indoc!("818181911112111")), 888911112111);
    assert_eq!(
        process_data_b(indoc!(
            "987654321111111
            811111111111119
            234234234234278
            818181911112111
            "
        )),
        3121910778619
    );
}
