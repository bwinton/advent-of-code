//-----------------------------------------------------
// Setup.

static INPUT: usize = 990_941;
static ZERO: u8 = b'0';

#[allow(dead_code)]
fn print_board(scores: &[u8], elf: &(usize, usize)) {
    for (i, score) in scores.iter().enumerate() {
        if i == elf.0 {
            print!("({}) ", score);
        } else if i == elf.1 {
            print!("[{}] ", score);
        } else {
            print!(" {}  ", score);
        }
    }
    println!();
}

fn step(scores: &mut Vec<u8>, elf: &mut (usize, usize)) {
    let new_score = scores[elf.0] + scores[elf.1];
    let mut chars = new_score
        .to_string()
        .chars()
        .map(|c| c as u8 - ZERO)
        .collect();
    scores.append(&mut chars);
    elf.0 = (elf.0 + scores[elf.0] as usize + 1) % scores.len();
    elf.1 = (elf.1 + scores[elf.1] as usize + 1) % scores.len();
}

fn process_data_a(start: usize) -> String {
    let mut scores: Vec<u8> = vec![3, 7];
    let mut elf = (0, 1);

    while start + 10 > scores.len() {
        step(&mut scores, &mut elf);
        // print_board(&scores, &(start, start+10));
    }
    scores[start..start + 10]
        .iter()
        .map(|&x| char::from(x + ZERO))
        .collect()
}

fn process_data_b(start: usize) -> usize {
    let target: Vec<u8> = format!("{:05}", start)
        .chars()
        .map(|c| c as u8 - ZERO)
        .collect();
    let target_len = target.len();
    let mut scores: Vec<u8> = vec![3, 7];
    let mut elf = (0, 1);
    let mut current = 0;

    while scores.len() < current + target_len {
        step(&mut scores, &mut elf);
    }

    while &scores[current..current + target_len] != target.as_slice() {
        current += 1;
        while scores.len() < current + target_len {
            step(&mut scores, &mut elf);
        }
    }
    current
    // 153723222 is too high.
}

//-----------------------------------------------------
// Questions.

q_impl!("14");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(9), "5158916779");
    assert_eq!(process_data_a(5), "0124515891");
    assert_eq!(process_data_a(18), "9251071085");
    assert_eq!(process_data_a(2018), "5941429882");
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(51_589), 9);
    assert_eq!(process_data_b(01_245), 5);
    assert_eq!(process_data_b(92_510), 18);
    assert_eq!(process_data_b(59_414), 2018);
}
