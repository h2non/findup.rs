fn plus_one(x: &int) -> int {
  *x + 1
}

fn main() {
  let y = 10;
  println!("Hola: {}", plus_one(y).to_string());
}