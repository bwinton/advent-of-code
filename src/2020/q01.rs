//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> usize {
    let numbers: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    for (index, num1) in numbers.iter().enumerate() {
        for num2 in numbers.clone().iter().skip(index) {
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    0
}

fn process_data_b(data: &str) -> usize {
    let numbers: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    for (index, num1) in numbers.iter().enumerate() {
        for (index2, num2) in numbers.clone().iter().enumerate().skip(index) {
            for num3 in numbers.clone().iter().skip(index2) {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "1721
979
366
299
675
1456"
        ),
        514579
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "1721
979
366
299
675
1456"
        ),
        241861950
    );
}
