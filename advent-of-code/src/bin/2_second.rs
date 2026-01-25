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
  fn test() {
    assert!(is_invalid(111));
  }
}

fn is_invalid(num: u64) -> bool {
  let mut digits = vec![];
  let mut num = num;
  while num != 0 {
    digits.push(num % 10);
    num = num / 10;
  }

  let l = digits.len();
  let all_factors = [2, 3, 5, 7, 11, 13, 17, 19]; // Can't go highter than this

  'outer: for factor in all_factors {
    if l % factor != 0 {
      continue;
    }

    // println!("Testing with {factor}");

    for start in 0..(l / factor) {
      let first = digits[start];
      for skip in 1..factor {
        let next = digits[start + skip * l / factor];

        if first != next {
          continue 'outer;
        }
      }
    }
    return true;
  }

  return false;
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
        // println!("{}", num);
        sum_invalids += num;
      }
    }
  }

  println!("{}", sum_invalids);
}

// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
