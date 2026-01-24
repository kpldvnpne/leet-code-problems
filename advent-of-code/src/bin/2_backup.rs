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
  fn test_invalid_of_len() {
    assert_eq!(invalid_of_len(2), 9);
    assert_eq!(invalid_of_len(4), 90);
    assert_eq!(invalid_of_len(0), 1);
  }

  #[test]
  #[should_panic]
  fn test_panic() {
    invalid_of_len(3);
  }

  #[test]
  fn test_invalid_under() {
    assert_eq!(invalid_under("11"), 1);
    assert_eq!(invalid_under("22"), 2);
    assert_eq!(invalid_under("33"), 3);

    assert_eq!(invalid_under("123123"), 9 + 90 + 20 + 3 + 1);

    println!("{}", invalid_under("1188511879"));
    println!("{}", invalid_under("1188511890"));
  }
}

fn invalid_of_len(len: u32) -> u64 {
  assert!(len % 2 == 0);

  if len == 0 {
    return 1;
  }

  let mut ans = 9;
  for _ in 1..(len / 2) {
    ans = ans * 10;
  }
  ans
}

fn invalid_under(num: &str) -> u64 {
  let num = num.as_bytes();
  let mut sum = 0;

  let start_len = 2;
  let mut cur_len = start_len;
  while cur_len < num.len() {
    sum += invalid_of_len(cur_len as u32);
    cur_len += 2;
  }

  if num.len() % 2 == 1 {
    return sum;
  }

  let mut i = 0;
  while i < num.len() / 2 {
    let prev_options = (num[i] - '0' as u8) as u64 + if i != 0 { 1 } else { 0 } - 1;
    sum += prev_options * 10u64.pow((num.len() / 2 - (i + 1)) as u32);

    i = i + 1;
  }

  for i in 0..num.len() / 2 {
    let j = i + num.len() / 2;
    if num[i] < num[j] {
      sum += 1;
      break;
    } else if num[i] > num[j] {
      break;
    } else {
      if i == 0 {
        sum += 1;
        break;
      }
    }
  }

  sum
}

fn main() {
  let line = read_line();
  let nums: Vec<Vec<&str>> = line.split(',').map(|it| it.split('-').collect()).collect();

  let mut n_invalids = 0;
  for pair in nums {
    let start = pair[0];
    let end = pair[1];

    // println!("{start}");
    let before_start = (start.parse::<u64>().unwrap() - 1).to_string();


    let for_end = invalid_under(end);
    let for_before_start = invalid_under(&before_start);

    println!("{} {} {}", for_end, for_before_start, for_end - for_before_start);
    n_invalids += for_end - for_before_start;
  }

  println!("{}", n_invalids);
}

// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
