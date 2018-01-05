//-----------------------------------------------------
// Setup.

use day;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::u64::MAX;

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    return String::from("5");
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    print!("Result = ");

    let mut hasher = Md5::new();
    // let input = "abc".as_bytes();
    let input = "abbhdwsy".as_bytes();
    let mut len = 0;

    for i in 0..MAX {
      hasher.input(input);
      hasher.input(i.to_string().as_bytes());

      let mut output = [0; 16]; // An MD5 is 16 bytes
      hasher.result(&mut output);
      let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
      if first_five == 0 {
        print!("{:x}", output[2]);
        len += 1
      }
      if len == 8 {
        break;
      }
      hasher.reset();
    }
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    print!("Result = ");

    // let input = "abc".as_bytes();
    let input = "abbhdwsy".as_bytes();

    let mut hasher = Md5::new();
    let mut password = [0xff as u8; 8];
    let mut len = 0;

    for i in 0..MAX {
      hasher.input(input);
      hasher.input(i.to_string().as_bytes());

      let mut output = [0; 16]; // An MD5 is 16 bytes
      hasher.result(&mut output);
      let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
      if first_five == 0 {
        let index = output[2] as usize;
        if index > 7 {
          hasher.reset();
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
      hasher.reset();
    }

    for value in password.iter() {
      print!("{:x}", value);
    }
    println!("");
  }
}
