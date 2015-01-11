# findup.rs [![Build Status](https://api.travis-ci.org/h2non/findup.rs.svg?branch=master)][travis]

A [Rust](http://rust-lang.org) small crate to find the first file matching in the current directory 
or the nearest ancestor directory up to root with Glob patterns support. Inspired in node's [findup](https://npmjs.org/package/findup) and Go [findup](https://github.com/h2non/findup)

**Note**: this is still a hacking-driven just-for-fun alpha package as 
result of a couple of hours learning and playing with Rust

## Usage

To use `findup`, add this to your `Cargo.toml` manifest
```toml
[dependencies]
findup = "0.1.0"
```

And add this to your crate root:
```rust
extern crate findup;
```

## Example

```rust
extern crate findup;

use std::path;
use findup::findup;

fn main() {
  let file: Path = findup("my-file.*");
  assert_eq!(file.exists(), true);
  
  if file.as_str() == Some(".") {
    println!("File path: {}", file.display());
  } else {
    println!("File not found");
  }
}
```

## License

MIT - Tomas Aparicio

[travis]: https://travis-ci.org/h2non/findup.rs
