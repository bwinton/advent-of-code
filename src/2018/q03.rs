//-----------------------------------------------------
// Setup.

static INPUT : &'static str = include_str!("data/q03.data");

fn process_data_a(_data: &str) -> i32 {
  0
}

fn process_data_b(_data: &str) -> i32 {
  0
}

//-----------------------------------------------------
// Questions.

q_impl!("3");

#[test]
fn test_a() {
  assert_eq!(process_data_a(""), 0);
}

#[test]
fn test_b() {
  assert_eq!(process_data_b(""), 0);
}
