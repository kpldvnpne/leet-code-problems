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
  let n = read_val_line::<usize>();
  let s = read_line();
  let s = s.trim().as_bytes();

  let mut found = false;
  for i in 0..(n-1) {
    if s[i] > s[i+1] {
      println!("YES");
      println!("{} {}", i+1, i+2);
      found = true;
      break;
    }
  }

  if !found {
    println!("NO");
  }
}
