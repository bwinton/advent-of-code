//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

static INPUT: &str = include_str!("data/q11.data");

fn blink(stones: &Vec<String>, seen: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut stones = stones.clone();
    let mut rv = vec![];
    let check = stones.join(" ");
    println!("Looking in {}", check);
    for key in seen.keys() {
        if check.contains(key) {
            println!("  Found key!!! {} in {}", check.len(), key.len());
        }
    }
    stones.reverse();
    while !stones.is_empty() {
        let stone = stones.pop().unwrap();
        // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        // print!("  {}: {} => ", i, stone);
        if stone == "0".to_owned() {
            rv.push("1".to_owned());
        } else if stone.len() % 2 == 0 {
            // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
            let clone = stone.clone();
            let (left, right) = clone.split_at(stone.len() / 2);
            // println!("  l:{:?}, r:{:?}", left, right);
            rv.push(left.to_owned());
            let mut right = right.trim_start_matches('0').to_owned();
            // println!("r0: {:?}, {}", right, right.is_empty());
            if right.is_empty() {
                // println!("r1: {:?}", right);
                right = "0".to_owned();
            }
            // println!("r2: {:?}", right);
            rv.push(right);
        } else {
            // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
            rv.push(format!("{}", stone.parse::<i64>().unwrap() * 2024));
        }
        // println!("{}", stone);
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut stones: Vec<String> = data.split_whitespace().map(|s| s.to_owned()).collect();
    let mut seen: HashMap<String, Vec<String>> = HashMap::new();
    for _i in 0..25 {
        println!("{:02}: {:?}", _i, stones.len());
        let next = blink(&stones, &seen);
        seen.insert(stones.join(" "), next.clone());
        stones = next;
    }
    // println!("25: {:?}", stones);
    stones.len()
}

fn process_data_b(data: &str) -> usize {
    let mut stones: Vec<String> = data.split_whitespace().map(|s| s.to_owned()).collect();
    let mut seen: HashMap<String, Vec<String>> = HashMap::new();
    for _i in 0..75 {
        println!("{:02}: {}", _i, stones.len());
        let next = blink(&stones, &seen);
        seen.insert(stones.join(" "), next.clone());
        stones = next;
    }
    // println!("25: {:?}", stones);
    stones.len()
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    // let data: Vec<String> = "0 1 10 99 999".split_whitespace().map(|s| s.to_owned()).collect();
    // let expected: Vec<String> = "1 2024 1 0 9 9 2021976".split_whitespace().map(|s| s.to_owned()).collect();
    // let data = blink(&data, &HashMap::new());
    // assert_eq!(data, expected);

    // let data: Vec<String> = "125 17".split_whitespace().map(|s| s.to_owned()).collect();
    // let expected: Vec<String> = "253000 1 7".split_whitespace().map(|s| s.to_owned()).collect();
    // let data = blink(&data, &HashMap::new());
    // // println!("data: {:?}", data);
    // assert_eq!(data, expected);

    // let expected: Vec<String> = "253 0 2024 14168".split_whitespace().map(|s| s.to_owned()).collect();
    // let data = blink(&data, &HashMap::new());
    // // println!("data: {:?}", data);
    // assert_eq!(data, expected);

    // let expected: Vec<String> = "512072 1 20 24 28676032".split_whitespace().map(|s| s.to_owned()).collect();
    // let data = blink(&data, &HashMap::new());
    // // println!("data: {:?}", data);
    // assert_eq!(data, expected);

    // let expected: Vec<String> = "512 72 2024 2 0 2 4 2867 6032".split_whitespace().map(|s| s.to_owned()).collect();
    // let data = blink(&data, &HashMap::new());
    // // println!("data: {:?}", data);
    // assert_eq!(data, expected);

    // let expected: Vec<String> = "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32".split_whitespace().map(|s| s.to_owned()).collect();
    // let data = blink(&data, &HashMap::new());
    // // println!("data: {:?}", data);
    // assert_eq!(data, expected);


    // let expected: Vec<String> = "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2".split_whitespace().map(|s| s.to_owned()).collect();
    // let data = blink(&data, &HashMap::new());
    // // println!("data: {:?}", data);
    // assert_eq!(data, expected);

    assert_eq!(process_data_a(indoc!("125 17")), 55312);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("")), 0);
}
