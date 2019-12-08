//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use std::fmt::Write;

static INPUT: &str = include_str!("data/q08.data");

fn process_data_a(data: &str) -> i32 {
    let mut found = (999, 0, 0);
    for chunk in &data.chars().chunks(25 * 6) {
        let chunk = chunk.collect::<Vec<char>>();
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;
        for character in chunk {
            match character {
                '0' => zeros += 1,
                '1' => ones += 1,
                '2' => twos += 1,
                _ => {}
            }
        }
        if zeros < found.0 {
            found = (zeros, ones, twos)
        }
    }

    found.1 * found.2
}

fn process_data_b(data: &str) -> String {
    let mut image: [[char; 25]; 6] = [['2'; 25]; 6];
    for chunk in &data.chars().chunks(25 * 6) {
        let chunk = chunk.collect::<Vec<char>>();
        for (i, character) in chunk.iter().enumerate() {
            if image[i / 25][i % 25] != '2' {
                continue;
            }
            image[i / 25][i % 25] = *character;
        }
    }
    let mut s = String::new();
    if let Err(e) = writeln!(s) {
        return format!("Error {}", e);
    }
    for line in &image {
        for character in line {
            if let Err(e) = write!(s, "{}", if *character == '1' { '*' } else { ' ' }) {
                return format!("Error {}", e);
            }
        }
        if let Err(e) = writeln!(s) {
            return format!("Error {}", e);
        }
    }
    s
}

//-----------------------------------------------------
// Questions.

q_impl!("8");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(""),
        "
                         
                         
                         
                         
                         
                         
"
    );
}
