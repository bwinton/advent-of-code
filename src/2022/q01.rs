//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q01.data");

fn process_data_a(data: &str) -> usize {
    let mut elf = 0;
    let mut calories = vec![];
    for line in data.lines() {
        if line.trim().is_empty() {
            calories.push(elf);
            elf = 0;
            continue;
        }
        elf += line.parse::<usize>().unwrap();
    }
    calories.push(elf);
    calories.into_iter().max().unwrap_or_default()
}

fn process_data_b(data: &str) -> usize {
    let mut elf = 0;
    let mut calories = vec![];
    for line in data.lines() {
        if line.trim().is_empty() {
            calories.push(elf);
            elf = 0;
            continue;
        }
        elf += line.parse::<usize>().unwrap();
    }
    calories.push(elf);
    calories.sort();
    calories.reverse();
    calories.into_iter().take(3).sum()
}

//-----------------------------------------------------
// Questions.

q_impl!("1");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
    "
        )),
        24000
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000
    "
        )),
        45000
    );
}
