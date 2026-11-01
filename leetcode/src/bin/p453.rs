pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![1, 2, 3];
    let expected = 3;
    let actual = Solution::min_moves(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![1, 1, 1];
    let expected = 0;
    let actual = Solution::min_moves(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let nums = vec![1,1,1000000000];
    let expected = 999999999;
    let actual = Solution::min_moves(nums);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn min_moves(mut nums: Vec<i32>) -> i32 {
      nums.sort_by(|a, b| b.cmp(a));

      let n = nums.len();

      let mut ans: i32 = 0;
      for i in 1..n {
        let diff = nums[i-1] - nums[i];
        let k = i as i32;
        ans += k * diff;
      }

      return ans;
    }
}
