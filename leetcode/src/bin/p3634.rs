pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![2, 1, 5];
    let k = 2;

    let expected = 1;
    let actual = Solution::min_removal(nums, k);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![1, 6, 2, 9];
    let k = 3;

    let expected = 2;
    let actual = Solution::min_removal(nums, k);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let nums = vec![4, 6];
    let k = 2;

    let expected = 0;
    let actual = Solution::min_removal(nums, k);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test609() {
    let nums = vec![51, 70, 9, 21];
    let k = 2;

    let expected = 2;
    let actual = Solution::min_removal(nums, k);
    assert_eq!(expected, actual);
  }
}

fn binary_find_max_within2k(a: &Vec<i32>, max_item: i32, mut left: usize, mut right: usize) -> usize {
  while left < right {
    let mid = (left + right + 1) / 2;
    if a[mid] <= max_item {
      left = mid;
    } else {
      right = mid - 1;
    }
  }

  return left;
}


impl Solution {
  pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    nums.sort_unstable();

    let mut min = n - 1;
    for i in 0..n {
      let other_idx = binary_find_max_within2k(&nums, k * nums[i], i, n - 1);
      let to_del = i + (n-1-other_idx);

      min = min.min(to_del);
    }

    let ans = min;
    return ans as i32;
  }
}
