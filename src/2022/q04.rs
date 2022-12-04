//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q04.data");

fn get_segment(elves: &mut std::str::Split<char>) -> Vec<usize> {
    elves
        .next()
        .unwrap()
        .splitn(2, '-')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let mut elves = line.split(',');
        let first = get_segment(&mut elves);
        let second = get_segment(&mut elves);
        if first[0] >= second[0] && first[1] <= second[1]
            || second[0] >= first[0] && second[1] <= first[1]
        {
            rv += 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        let mut elves = line.split(',');
        let first = get_segment(&mut elves);
        let second = get_segment(&mut elves);
        if first[0] <= second[0] && first[1] >= second[0]
            || first[0] >= second[0] && first[0] <= second[1]
        {
            rv += 1;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "
        )),
        2
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "
        )),
        4
    );
}
