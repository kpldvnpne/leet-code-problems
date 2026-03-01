pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let n = "32".to_string();
    let expected = 3;
    let actual = Solution::min_partitions(n);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let n = "82734".to_string();
    let expected = 8;
    let actual = Solution::min_partitions(n);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let n = "27346209830709182346".to_string();
    let expected = 9;
    let actual = Solution::min_partitions(n);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
      let mut min_ans = 0;
      for c in n.chars() {
        let digit = c as u8 - '0' as u8;
        min_ans = min_ans.max(digit);
      }

      return min_ans as i32;
    }
}
