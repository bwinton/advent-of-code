macro_rules! q_vec {
  ( $( $x:ident ),* ) => {
    {
      let temp_vec:Vec<Box<day::Day>> = vec!(
      $(
        Box::new($x::Q),
      )*
      );
      temp_vec
    }
  };
}

// From https://play.rust-lang.org/?gist=0cbc09e0fc41016f5f5c240d088a4410&version=stable
macro_rules! define_iterator {
  ($itname:ident ($(&$name:ident : $ty:ty = $e:expr),*) -> Option<$rty:ty> { $($body:tt)* }) => {
    struct $itname {
      $($name : $ty),*
    }

    impl std::default::Default for $itname {
      fn default() -> Self {
        $itname {
          $($name : $e),*
        }
      }
    }

    impl Iterator for $itname {
      type Item = $rty;
      fn next(&mut self) -> Option<Self::Item> {
        $(let $name : &mut $ty = &mut self.$name;)*
        $($body)*
      }
    }
  }
}
