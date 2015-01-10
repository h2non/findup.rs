#![doc(html_logo_url = "http://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "http://www.rust-lang.org/favicon.ico")]
#![allow(unstable)]
#[warm(unstable)]

extern crate glob;

use std::os;
use std::path;
use glob::glob;
use std::io::fs:: PathExtensions;

fn abs_path(path: &str) -> Path {
  os::getcwd().unwrap().join(&Path::new(path))
}

fn full_path(base: Path, path: &str) -> Path {
  base.join(&Path::new(path))
}

fn base_path(partial: &str) -> Path {
  let cwd = os::getcwd().unwrap();
  return match partial.len() {
    0 => cwd,
    _ => cwd.join(&Path::new(partial))
  }
}

fn lookup(base: &str, pattern: &str) -> Path {
  let path = full_path(base_path(base), pattern);
  let path_str = path.as_str().unwrap();

  if !path.exists() {
    return Path::new("");
  }
  
  let mut buf: Vec<Path> = glob(pattern).collect();

  if buf.is_empty() {
    return lookup("..", pattern);
  } else {
    return buf.get(0).unwrap().clone();
  }
}

pub fn findup(pattern: &str) -> Path {
  lookup("", pattern)
}
