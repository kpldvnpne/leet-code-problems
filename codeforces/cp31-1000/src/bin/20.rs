fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

use std::fmt::Debug;
use std::str::FromStr;
#[allow(dead_code)]
fn read_val_line<T: FromStr>() -> T
  where <T as FromStr>::Err : Debug
{
  read_line().trim().parse().unwrap()
}

#[allow(dead_code)]
fn read_vector_line<T: FromStr>() -> Vec<T>
  where <T as FromStr>::Err : Debug
{
  read_line().trim().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

#[allow(dead_code)]
fn read_string_array() -> Vec<String> {
    read_line().trim().split_ascii_whitespace().map(|it| it.trim().to_string()).collect()
}

fn main() {
  let t = read_val_line::<u32>();
  for _ in 0..t {
    let [a, b] = read_vector_line::<u32>()[..2] else { panic!() };
    let ans = if a == 0 {
      if b == 0 { 2 } else { 1 }
    } else {
      let min = a;
      let cur_xor = match a % 4 {
        0 => 0,
        1 => a - 1,
        2 => 1,
        3 => a,
        _ => unreachable!()
      };
      if b == cur_xor {
        min
      } else if a == (b ^ cur_xor) {
        min + 2
      } else {
        min + 1
      }
    };

    println!("{}", ans);
  }
}
