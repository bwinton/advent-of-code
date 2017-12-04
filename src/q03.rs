//-----------------------------------------------------
// Setup.

use day;

static INPUT : i32 = 265_149;

fn process_data_a(data: i32) -> i32 {
  if data == 1 { return 0; }
  let numbers = 0..;
  let mut rv = 0;
  for i in numbers {
    let block = 2 * i + 1;
    if block*block >= data {
      let remainder = data - (block-2)*(block-2);
      let low = i;
      let high = 2*i;
      let mut seesaw = (low..high).chain(high-1..low+1).cycle().skip(remainder as usize);

      println!("i:{}, {}<->{}, r:{}, d:{}!", i, low, high, remainder, data);
      rv = seesaw.next().unwrap();
      break;
    }
  }
  rv
}

fn process_data_b(_data: i32) -> i32 {
  // if data == 1 { return 0; }
  // let numbers = 0..;
  // let mut rv = 0;
  // for i in numbers {
  //   let block = 2 * i + 1;
  //   if block*block >= data {
  //     let remainder = data - (block-2)*(block-2);
  //     let low = i;
  //     let high = 2*i;
  //     let mut seesaw = (low..high).chain(high-1..low+1).cycle().skip(remainder as usize);
  //
  //     println!("i:{}, {}<->{}, r:{}, d:{}!", i, low, high, remainder, data);
  //     rv = seesaw.next().unwrap();
  //     break;
  //   }
  // }
  // rv
  0
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl day::Day for Q {
  fn number(&self) -> String {
    String::from("3")
  }

  fn a(&self) {
    print!("{}A: ", self.number());
    let result = process_data_a(INPUT);
    println!("Result = {}", result);
  }

  fn b(&self) {
    print!("{}B: ", self.number());
    let result = process_data_b(INPUT);
    println!("Result = {}", result);
  }
}

#[test]
fn a() {
  assert_eq!(process_data_a(1), 0);
  assert_eq!(process_data_a(12), 3);
  assert_eq!(process_data_a(23), 2);
  assert_eq!(process_data_a(1024), 31);
}

#[test]
fn b() {
  assert_eq!(process_data_b(1), 1);
  assert_eq!(process_data_b(2), 1);
  assert_eq!(process_data_b(3), 2);
  assert_eq!(process_data_b(4), 4);
  assert_eq!(process_data_b(5), 5);
  assert_eq!(process_data_b(6), 10);
}
