use itertools::Itertools;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q03.data");

fn process_data_a(data: &str) -> i32 {
    let mut bits = vec![];
    let mut len = 0;
    for line in data.lines() {
        len += 1;
        for (i, bit) in line.chars().enumerate() {
            if bits.len() <= i {
                bits.push(0);
            }
            if bit == '1' {
                bits[i] += 1;
            }
        }
    }
    let bits = bits
        .iter()
        .map(|&digit| if digit > (len / 2) { '1' } else { '0' })
        .join("");
    let gamma = i32::from_str_radix(&bits, 2).unwrap();
    let epislon = !gamma & ((1 << bits.len()) - 1);
    // println!("{}x{} = {}", gamma, epislon, gamma*epislon);
    gamma * epislon
}

fn get_value(lines: &[(i32, &str)], smaller: bool) -> i32 {
    let mut curr = Vec::from(lines);
    for i in 0..lines[0].1.len() {
        let mid = curr[curr.len() / 2];
        let retain = mid.1.chars().nth(i).unwrap();
        let index = curr.partition_point(|&(_, line)| line.chars().nth(i).unwrap() == '0');
        curr = if smaller ^ (retain == '0') {
            curr[..index].to_vec()
        } else {
            curr[index..].to_vec()
        };
        if curr.len() == 1 {
            return curr[0].0;
        }
    }
    0
}

fn process_data_b(data: &str) -> i32 {
    let lines: Vec<_> = data
        .lines()
        .map(|x| (i32::from_str_radix(x, 2).unwrap(), x))
        .sorted()
        .collect();
    let og_value = get_value(&lines, false);
    let co_value = get_value(&lines, true);
    // println!("{} x {} = {}", og_value, co_value, og_value * co_value);
    og_value * co_value
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
    "
        )),
        198
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
    "
        )),
        230
    );
}
