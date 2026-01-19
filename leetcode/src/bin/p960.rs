pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let strs = ["babca", "bbazb"].iter().map(|it| it.to_string()).collect();

    let expected = 3;
    let actual = Solution::min_deletion_size(strs);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let strs = ["edcba"].iter().map(|it| it.to_string()).collect();

    let expected = 3;
    let actual = Solution::min_deletion_size(strs);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let strs = ["ghi", "def", "abc"].iter().map(|it| it.to_string()).collect();

    let expected = 0;
    let actual = Solution::min_deletion_size(strs);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
      let strs: Vec<&[u8]> = strs.iter().map(|it| it.as_bytes()).collect();

      let l = strs[0].len();
      let mut dp = vec![1; l];

      let mut max = 1;
      for j in 1..l {
        for k in (0..j).rev() {
          if dp[j] > dp[k] + 1 {
            continue; // Skip if the possible increase is not able to increase the current max
          }
          let mut in_order = true;
          for str in &strs {
            if str[k] > str[j] {
              in_order = false;
              break;
            }
          }

          if in_order {
            dp[j] = std::cmp::max(dp[j], dp[k] + 1);
          }
        }

        max = std::cmp::max(max, dp[j]);
      }

      return (l - max) as i32;
    }
}
