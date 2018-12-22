//-----------------------------------------------------
// Setup.

use std::collections::HashSet;

static INPUT: &'static str = include_str!("data/q21.data");

fn step(input: i64) -> i64 {
    let init = 707_129;
    let perturb = 65_899;
    let mut bytes = input | 0x10000;
    let mut hash = init;
    while bytes != 0 {
        hash += bytes & 0xFF;
        hash &= 0xFF_FFFF;
        hash *= perturb;
        hash &= 0xFF_FFFF;
        bytes >>= 8;
    }
    hash
}

fn process_data_a(_data: &str) -> i64 {
    step(0)
}

fn process_data_b(_data: &str) -> i64 {
    let mut results = HashSet::new();
    let mut last_result = -1;
    let mut r5 = 0;
    loop {
        let mut r3 = r5 | 65_536;
        r5 = 707_129;
        loop {
            r5 += r3 & 255;
            r5 &= 16_777_215;
            r5 *= 65_899;
            r5 &= 16_777_215;
            if 256 > r3 {
                break;
            }
            let mut r2 = 0;
            while (r2 + 1) * 256 <= r3 {
                r2 += 1;
            }
            r3 = r2;
        }

        if !results.contains(&r5) {
            results.insert(r5);
            last_result = r5;
        } else {
            return last_result;
        }
    }
    // 16776149 is too high.
    // 12502875
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    assert_eq!(process_data_a(""), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b(""), 0);
}
