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

fn is_empty(char: u8) -> bool {
  char == '.' as u8
}

fn is_block(char: u8) -> bool {
  char == '#' as u8
}

fn get_total_action1(n: usize, s: &[u8]) -> i32 {
  let mut total_action1 = 0;
  let mut rolling_len = 0;
  let mut previous_was_empty = false;
  for i in 0..n {
    let char = s[i];

    if is_block(char) {
      previous_was_empty = false;
      continue;
    } else {
      if previous_was_empty {
        rolling_len += 1;
        if rolling_len > 2 {
          return 2;
        }
      } else {
        total_action1 += rolling_len;
        rolling_len = 1;
      }
      previous_was_empty = true;
    }
  }
  total_action1 += rolling_len;
  return total_action1;
}

fn main() {
  let t: i32 = read_val_line();
  for _ in 0..t {
    let n: usize = read_val_line();
    let s = read_line();
    let s = s.trim().as_bytes();

    let n_action1 = get_total_action1(n, s);

    println!("{n_action1}");
  }
}
