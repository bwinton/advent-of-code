//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> u32 {
    let mut sum = 0;
    for line in data.lines() {
        let weight: u32 = line.parse().unwrap();
        sum += weight / 3 - 2;
    }
    sum
}

fn process_data_b(data: &str) -> u32 {
    let mut sum = 0;
    for line in data.lines() {
        let weight: u32 = line.parse().unwrap();
        let mut fuel: i32 = weight as i32 / 3 - 2;
        while fuel > 0 {
            sum += fuel as u32;
            fuel = fuel as i32 / 3 - 2;
        }
    }
    sum
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    assert_eq!(process_data_a("12"), 2);
    assert_eq!(process_data_a("14"), 2);
    assert_eq!(
        process_data_a(
            "12
14"
        ),
        4
    );
}

#[test]
fn b() {
    assert_eq!(process_data_b("14"), 2);
    assert_eq!(process_data_b("1969"), 966);
    assert_eq!(
        process_data_b(
            "14
1969"
        ),
        968
    );
}
