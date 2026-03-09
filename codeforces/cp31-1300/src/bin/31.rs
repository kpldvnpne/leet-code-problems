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

fn get_letter_other_than(c1: char, c2: char) -> char {
  for i in 0..26 {
    let c = ('a' as u8 + i) as char;
    if c != c1 && c != c2 {
      return c;
    }
  }
  unreachable!();
}

fn main() {
  let s = read_line();
  let mut s = s.trim().chars().collect::<Vec<_>>();
  let n = s.len();

  let mut new_s = String::new();
  new_s.push(s[0]);

  for i in 1..n-1 {
    let prev = s[i-1];
    let cur = s[i];
    let next = s[i+1];
    if cur == prev {
      s[i] = get_letter_other_than(prev, next);
    }
  }

  if n >= 2 {
    let prev = s[n-2];
    let cur = s[n-1];
    if cur == prev {
      s[n-1] = get_letter_other_than(cur, prev);
    }
  }

  let new_s = s.iter().collect::<String>();
  println!("{}", new_s);
}
