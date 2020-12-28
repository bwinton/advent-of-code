//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q25.data");

fn process_data_a(data: &str) -> usize {
    let mut lines = data.lines();
    let card_key: usize = lines.next().unwrap().parse().unwrap();
    let door_key: usize = lines.next().unwrap().parse().unwrap();

    let mut value = 1;
    let mut subject_number = 7;
    let mut loop_size = 0;
    while value != card_key {
        value *= subject_number;
        value %= 20201227;
        loop_size += 1;
    }
    // println!("card loop size = {}", loop_size);

    value = 1;
    subject_number = door_key;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    // println!("Final value = {}", value);
    value
}

fn process_data_b(_data: &str) -> usize {
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("25");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "5764801
            17807724"
        )),
        14_897_079
    );
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
