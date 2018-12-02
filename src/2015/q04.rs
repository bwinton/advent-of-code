//-----------------------------------------------------
// Setup.

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::i32::MAX;

static INPUT: &'static str = "bgvyzdsv";

fn process_data(data: &str, zeroes: usize) -> i32 {
    let mut hasher = Md5::new();
    let input = data.as_bytes();

    for i in 0..MAX {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);
        let mut first = 0;
        for value in output.iter().take(zeroes / 2) {
            first += i32::from(*value);
        }
        if zeroes % 2 == 1 {
            first += i32::from(output[zeroes / 2] >> 4);
        }
        if first == 0 {
            return i;
        }
        hasher.reset();
    }
    -1
}

fn process_data_a(data: &str) -> i32 {
    process_data(data, 5)
}

fn process_data_b(data: &str) -> i32 {
    process_data(data, 6)
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn test_a() {
    assert_eq!(process_data_a("abcdef"), 609043);
    assert_eq!(process_data_a("pqrstuv"), 1048970);
}

#[test]
fn test_b() {
    assert_eq!(process_data_b("abcdef"), 6742839);
    assert_eq!(process_data_b("pqrstuv"), 5714438);
}
