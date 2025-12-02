//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q02.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for range in data.trim().split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        for id in start..=end {
            let str_id = format!("{id}");
            if str_id[..str_id.len() / 2] == str_id[str_id.len() / 2..] {
                rv += id;
            }
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;

    for range in data.trim().split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        for id in start..=end {
            let str_id = format!("{id}");
            for split in 1..=str_id.len() / 2 {
                let prefix = &str_id[..split];
                let check = prefix.repeat(str_id.len() / split);
                if str_id == check {
                    rv += id;
                    break;
                }
            }
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("2");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        )),
        1227775554
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("11-22")), 33);
    assert_eq!(process_data_b(indoc!("95-115")), 210);
    assert_eq!(
        process_data_b(indoc!(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        )),
        4174379265
    );
}
