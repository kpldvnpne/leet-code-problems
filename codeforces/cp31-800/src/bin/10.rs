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

// x = 1 to 10 and y = 1 to 10
fn get_value(x: i32, y: i32) -> i32 {
  let x = if x > 5 { 11 - x } else { x };
  let y = if y > 5 { 11 - y } else { y };

  // Return the minimum which determines the value
  return std::cmp::min(x, y);
}

fn main() {
  let t: i32 = read_val_line();
  for _ in 0..t {
    let mut total_points = 0;
    for y in 0..10 {
      let s = read_line();
      let s = s.as_bytes();
      for x in 0..10 {
        if s[x] == 'X' as u8 {
          total_points += get_value(x as i32 + 1, y + 1);
        }
      }
    }

    println!("{total_points}");
  }
}
