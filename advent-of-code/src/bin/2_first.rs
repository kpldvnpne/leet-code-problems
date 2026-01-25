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

fn is_invalid(num: u64) -> bool {
  let mut digits = vec![];
  let mut num = num;
  while num != 0 {
    digits.push(num % 10);
    num = num / 10;
  }

  if digits.len() % 2 == 1 {
    return false;
  }

  for i in 0..digits.len()/2 {
    let j = i + digits.len() / 2;
    if digits[i] != digits[j] {
      return false;
    }
  }

  return true;
}

fn main() {
  let line = read_line();
  let nums: Vec<Vec<u64>> = line
    .split(',')
    .map(|it|
      it.split('-')
        .map(|it| {
          // println!("{it}");
          it.trim().parse().unwrap()
        }).collect()
    ).collect();

  let mut sum_invalids = 0;
  for pair in nums {
    let start = pair[0];
    let end = pair[1];

    for num in start..=end {
      if is_invalid(num) {
        sum_invalids += num;
      }
    }
  }

  println!("{}", sum_invalids);
}

// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
