//-----------------------------------------------------
// Setup.

use aoc::Day;
use md5::{Digest, Md5};

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("5")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        print!("Result = ");

        let mut hasher = Md5::new();
        // let input = "abc".as_bytes();
        let input = b"abbhdwsy";
        let mut len = 0;

        for i in 0..u64::MAX {
            hasher.update(input);
            hasher.update(i.to_string().as_bytes());

            let output = hasher.finalize_reset();
            let first_five =
                i32::from(output[0]) + i32::from(output[1]) + i32::from(output[2] >> 4);
            if first_five == 0 {
                print!("{:x}", output[2]);
                len += 1
            }
            if len == 8 {
                break;
            }
        }
        println!();
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        print!("Result = ");

        // let input = "abc".as_bytes();
        let input = b"abbhdwsy";

        let mut hasher = Md5::new();
        let mut password = [0xff_u8; 8];
        let mut len = 0;

        for i in 0..u64::MAX {
            hasher.update(input);
            hasher.update(i.to_string().as_bytes());

            let output = hasher.finalize_reset();
            let first_five =
                i32::from(output[0]) + i32::from(output[1]) + i32::from(output[2] >> 4);
            if first_five == 0 {
                let index = output[2] as usize;
                if index > 7 {
                    continue;
                }
                if password[index] == 0xff {
                    password[index] = output[3] >> 4;
                    len += 1;
                }
            }
            if len == 8 {
                break;
            }
        }

        for value in &password {
            print!("{:x}", value);
        }
        println!();
    }
}
