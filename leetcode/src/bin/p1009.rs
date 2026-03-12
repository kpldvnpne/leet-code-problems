pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let n = 5;
    let expected = 2;
    let actual = Solution::bitwise_complement(n);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut should_flip = false;
        let mut ans = 0;
        for bit_pos in (0..32).rev() {
            if should_flip {
                let bit = (n >> bit_pos) & 1;
                ans = (ans << 1) | (1 ^ bit);
            } else {
                if (n >> bit_pos) & 1 == 1 {
                    should_flip = true;
                }
            }
        }
        return ans;
    }
}
