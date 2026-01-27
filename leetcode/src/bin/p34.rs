pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;

    let expected = vec![3, 4];
    let actual = Solution::search_range(nums, target);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 6;

    let expected = vec![-1, -1];
    let actual = Solution::search_range(nums, target);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let nums = vec![];
    let target = 6;

    let expected = vec![-1, -1];
    let actual = Solution::search_range(nums, target);

    assert_eq!(expected, actual);
  }
}

fn binary_search_left(arr: &[i32], target: i32) -> i32 {
  let n = arr.len();
  if n == 0 {
    return -1;
  }

  let mut left = 0;
  let mut right = n - 1;
  while left != right {
    let mid = (left + right) / 2;
    if arr[mid] >= target {
      right = mid;
    } else {
      left = mid + 1;
    }
  }

  if arr[left] == target {
    return left as i32;
  } else {
    return -1;
  }
}

fn binary_search_right(arr: &[i32], target: i32) -> i32 {
  let n = arr.len();
  if n == 0 {
    return -1;
  }

  let mut left = 0;
  let mut right = n - 1;
  while left != right {
    let mid = (left + right + 1) / 2;
    if arr[mid] <= target {
      left = mid;
    } else {
      right = mid - 1;
    }
  }

  if arr[left] == target {
    return left as i32;
  } else {
    return -1;
  }
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
      return vec![binary_search_left(&nums, target), binary_search_right(&nums, target)];
    }
}
