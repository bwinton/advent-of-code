//-----------------------------------------------------
// Setup.

static INPUT: &str = "248345-746315";

fn check_a(i: usize) -> bool {
    let str_i = i.to_string();
    let mut last_letter = ' ';
    let mut valid = false;
    for letter in str_i.chars() {
        if letter < last_letter {
            return false;
        }
        if letter == last_letter {
            valid = true
        }
        last_letter = letter;
    }
    valid
}

fn process_data_a(data: &str) -> usize {
    let data: Vec<usize> = data
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut total = 0;
    for i in data[0]..data[1] {
        if check_a(i) {
            total += 1;
        }
    }
    total
}

fn check_b(i: usize) -> bool {
    let str_i = i.to_string();
    let mut last_letter = ' ';
    let mut valid = false;
    let mut group_size = 1;
    for letter in str_i.chars() {
        if letter < last_letter {
            return false;
        }
        if letter == last_letter {
            group_size += 1;
        } else {
            if group_size == 2 {
                valid = true;
            }
            group_size = 1;
        }
        last_letter = letter;
    }
    if group_size == 2 {
        valid = true;
    }
    valid
}

fn process_data_b(data: &str) -> i32 {
    let data: Vec<usize> = data
        .split('-')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut total = 0;
    for i in data[0]..data[1] {
        if check_b(i) {
            total += 1;
        }
    }
    total
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    assert_eq!(check_a(111_111), true);
    assert_eq!(check_a(223_450), false);
    assert_eq!(check_a(123_789), false);
}

#[test]
fn b() {
    assert_eq!(check_b(112_233), true);
    assert_eq!(check_b(123_444), false);
    assert_eq!(check_b(111_122), true);
    assert_eq!(check_b(112_222), true);
}
