#![feature(macro_rules)]

extern crate findup;

use std::os;
use std::io;
use std::io::TempDir;
use std::io::fs:: PathExtensions;

use findup::findup;

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
  assert_eq!(file.exists(), true);
}

#[test]
fn find_glob_pattern_in_current_directory() {
  let file = findup("*.toml");
  assert_eq!(file.exists(), true);
}

// to do: fix
#[test]
fn cannot_find_file() {
  let file = findup("invalid");
  assert_eq!(file.exists(), true);
}
