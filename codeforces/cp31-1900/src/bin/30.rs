fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

fn main() {
  let t = read_val_line::<usize>();
  for _ in 0..t {
    let s = read_line();
    let s = s.trim().as_bytes();

    let mut stack: Vec<u8> = vec![];
    let mut n_moves = 0;
    for c in s {
      if let Some(top) = stack.last() && *top != *c {
        n_moves += 1;
        stack.pop();
      } else {
        stack.push(*c);
      }
    }

    if n_moves % 2 == 0 {
      println!("NET");
    } else {
      println!("DA");
    }
  }
}
