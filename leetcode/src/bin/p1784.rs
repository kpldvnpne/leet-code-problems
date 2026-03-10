pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let s= "1001".to_string();
    let expected = false;
    let actual = Solution::check_ones_segment(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s= "110".to_string();
    let expected = true;
    let actual = Solution::check_ones_segment(s);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
      let mut found_0 = false;
      for c in s.chars() {
        if c == '0' {
          found_0 = true;
        } else if c == '1' && found_0 {
          return false;
        }
      }

      return true;
    }
}
