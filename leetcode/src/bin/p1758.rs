pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let s = "0100".to_string();
    let expected = 1;
    let actual = Solution::min_operations(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s = "10".to_string();
    let expected = 0;
    let actual = Solution::min_operations(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let s = "1111".to_string();
    let expected = 2;
    let actual = Solution::min_operations(s);

    assert_eq!(expected, actual);
  }
}

fn ops_to_start_with(s: &[u8], c: char) -> i32 {
  let other = if c == '0' { '1' } else { '0' };
  let c= c as u8;
  let other = other as u8;

  let mut n_ops = 0;
  for i in (0..s.len()).step_by(2) {
    if s[i] != c {
      n_ops += 1;
    }
  }

  for i in (1..s.len()).step_by(2) {
    if s[i] != other {
      n_ops += 1;
    }
  }

  return n_ops;
}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
      let s = s.as_bytes();
      return ops_to_start_with(s, '0').min(ops_to_start_with(s, '1'));
    }
}
