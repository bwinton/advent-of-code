pub trait Day {
  fn number(&self) -> String;
  fn a(&self, use_test_data: bool);
  fn b(&self, use_test_data: bool);
}
