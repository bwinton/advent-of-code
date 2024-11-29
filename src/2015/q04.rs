//-----------------------------------------------------
// Setup.

use crypto::{digest::Digest, md5::Md5};

static INPUT: &str = "bgvyzdsv";

fn process_data(data: &str, zeroes: usize) -> i32 {
    let mut hasher = Md5::new();
    let input = data.as_bytes();

    for i in 0..i32::MAX {
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
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("abcdef"), 609_043);
    assert_eq!(process_data_a("pqrstuv"), 1_048_970);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("abcdef"), 6_742_839);
    assert_eq!(process_data_b("pqrstuv"), 5_714_438);
}
