pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let n = 43261596;
    let expected = 964176192;
    let actual = Solution::reverse_bits(n);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let n = 2147483644;
    let expected = 1073741822;
    let actual = Solution::reverse_bits(n);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
      let mut new_num = 0;
      for i in 0..32 {
        new_num = new_num << 1 | ((n >> i) & 1);
      }

      return new_num;
    }
}
