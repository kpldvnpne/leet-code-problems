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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let ans = -1 % 100;
    println!("{}", ans);
    let ans = -100 % 100;
    println!("{}", ans);
    let ans = -101 % 100;
    println!("{}", ans);
  }

  #[test]
  fn test2() {
    assert_eq!(to_lock_pos(-1), 99);
    assert_eq!(to_lock_pos(99), 99);
    assert_eq!(to_lock_pos(100), 0);
    assert_eq!(to_lock_pos(-100), 0);
    assert_eq!(to_lock_pos(-101), 99);
    assert_eq!(to_lock_pos(-200), 0);
  }
}

fn to_lock_pos(steps: i32) -> i32 {
  if steps < 0 {
    let times = steps / -100 + if steps % 100 == 0 {0} else {1};
    return times * 100 + steps;
  } else {
    let times = steps / 100;
    return steps - times * 100;
  }
}

fn main() {
  let start = 50;
  let mut cur_pos = start;
  let mut ans = 0;
  loop {
    let line = read_line();
    let line = line.trim();
    if line.is_empty() {
      break;
    }

    let (direction, distance) = line.split_at(1);
    let distance = distance.parse::<i32>().unwrap();

    if direction == "L" {
      cur_pos -= distance;
    } else {
      cur_pos += distance;
    }

    cur_pos = to_lock_pos(cur_pos);

    if cur_pos == 0 {
      ans += 1;
    }
  }

  println!("{ans}");
}
