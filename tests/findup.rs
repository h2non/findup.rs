#![feature(macro_rules)]

extern crate findup;

use findup::findup;
use std::os;
use std::io;
use std::io::TempDir;

macro_rules! assert_eq { 
  ($e1:expr, $e2:expr) => (
    if $e1 != $e2 {
      panic!("{} != {}", stringify!($e1), stringify!($e2))
    }
  )
}

fn mk_file(path: &str, directory: bool) {
  if directory {
    io::fs::mkdir(&Path::new(path), io::USER_RWX).unwrap();
  } else {
    io::File::create(&Path::new(path)).unwrap();
  }
}

#[test]
fn find_file_in_current_directory() {
  let file = findup("Cargo.toml");
  assert_eq!(file.exists(), true)
}
