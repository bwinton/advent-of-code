//-----------------------------------------------------
// Common Header.

pub fn select(arg: &String) {
  match arg.as_ref() {
    "5a" => a(),
    "5b" => b(),
    "5" | "*" => {a(); b()},
    _ => ()
  }
}

//-----------------------------------------------------
// Setup.

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::u64::MAX;

//-----------------------------------------------------
// Questions.

fn a() {
  print!("5A: Result = ");

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

fn b() {
  print!("5B: ");
  println!("Result = ");
}
