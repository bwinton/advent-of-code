use itertools::Itertools;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q03.data");

fn process_data_a(data: &str) -> i32 {
    let mut bits = vec![];
    let mut len = 0;
    for line in data.lines() {
        // Do something
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
    // println!("{:?} {}", bits, len);
    let bits = bits.iter().map(|&digit| if digit > (len / 2) {'1'} else {'0'}).join("");
    let gamma = i32::from_str_radix(&bits, 2).unwrap();
    let epislon = !gamma & ((1<<bits.len()) - 1);
    // println!("{}x{} = {}", gamma, epislon, gamma*epislon);
    gamma * epislon
}

fn process_data_b(data: &str) -> usize {
    let mut og_lines: Vec<_> = data.lines().collect();
    let mut co_lines = og_lines.clone();
    let mut og_value = 0;
    let mut co_value = 0;
    for i in 0..data.lines().next().unwrap().len() {
        let mut zero_lines = vec![];
        let mut one_lines = vec![];
        for &line in og_lines.iter() {
            if line.chars().nth(i).unwrap() == '1' {
                one_lines.push(line);
            } else {
                zero_lines.push(line);
            }
        }

        if one_lines.len() >= zero_lines.len() {
            og_lines = one_lines;
        } else {
            og_lines = zero_lines;
        }
        if og_lines.len() == 1 {
            for digit in og_lines[0].chars() {
                if digit == '1' {
                    og_value += 1;
                }
                og_value *= 2;
            }
            og_value /= 2;
            break;
        }
    }
    for i in 0..data.lines().next().unwrap().len() {
        let mut zero_lines = vec![];
        let mut one_lines = vec![];
        for &line in co_lines.iter() {
            if line.chars().nth(i).unwrap() == '1' {
                one_lines.push(line);
            } else {
                zero_lines.push(line);
            }
        }

        if one_lines.len() < zero_lines.len() {
            co_lines = one_lines;
        } else {
            co_lines = zero_lines;
        }
        if co_lines.len() == 1 {
            for digit in co_lines[0].chars() {
                if digit == '1' {
                    co_value += 1;
                }
                co_value *= 2;
            }
            co_value /= 2;
            break;
        }
    }
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
