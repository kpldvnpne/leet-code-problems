pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let s = "110".to_string();
    let k = 1;
    let expected = 1;
    let actual = Solution::min_operations(s, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s = "0101".to_string();
    let k = 3;
    let expected = 2;
    let actual = Solution::min_operations(s, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let s = "101".to_string();
    let k = 2;
    let expected = -1;
    let actual = Solution::min_operations(s, k);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
      let n_0s = s.chars().filter(|&c| c == '0').count();

      if n_0s == 0 {
        return 0;
      }

      let k = k as usize;

      if k == s.len() {
        if n_0s == k {
          return 1;
        } else {
          return -1;
        }
      }

      if n_0s % k == 0 {
        return (n_0s / k) as i32;
      }



      return _;
    }
}
