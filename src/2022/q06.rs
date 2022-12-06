//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

static INPUT: &str = include_str!("data/q06.data");

fn process(data: &str, size: usize) -> usize {
    let mut rv = 0;

    let data: Vec<char> = data.chars().collect();
    for (i, chars) in data.windows(size).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(chars);
        if set.len() == size {
            rv = i + size;
            break;
        }
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    process(data, 4)
}

fn process_data_b(data: &str) -> usize {
    process(data, 14)
}

//-----------------------------------------------------
// Questions.

q_impl!("6");

#[test]
fn a() {
    assert_eq!(process_data_a(indoc!("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
    assert_eq!(process_data_a(indoc!("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
    assert_eq!(process_data_a(indoc!("nppdvjthqldpwncqszvftbrmjlhg")), 6);
    assert_eq!(
        process_data_a(indoc!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
        10
    );
    assert_eq!(
        process_data_a(indoc!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
        11
    );
}

#[test]
fn b() {
    assert_eq!(process_data_b(indoc!("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
    assert_eq!(process_data_b(indoc!("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
    assert_eq!(process_data_b(indoc!("nppdvjthqldpwncqszvftbrmjlhg")), 23);
    assert_eq!(
        process_data_b(indoc!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
        29
    );
    assert_eq!(
        process_data_b(indoc!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
        26
    );
}
