pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;

    let expected = 4;
    let actual = Solution::search(nums, target);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 3;

    let expected = -1;
    let actual = Solution::search(nums, target);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let nums = vec![1];
    let target = 0;

    let expected = -1;
    let actual = Solution::search(nums, target);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_intermediate() {
    let nums = vec![0, 1, 2, 3, 4];

    let expected = 0;
    let actual = find_partition(&nums);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_intermediate2() {
    let nums = vec![1, 2, 3, 4, 0];

    let expected = 4;
    let actual = find_partition(&nums);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_intermediate3() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];

    let expected = 4;
    let actual = find_partition(&nums);
    assert_eq!(expected, actual);
  }
}

fn find_partition(nums: &Vec<i32>) -> usize {
  let n = nums.len();

  if nums[0] < nums[n-1] {
    return 0;
  }

  let mut left = 0;
  let mut right = n-1;

  while left != right {
    let mid = (left + right) / 2;
    if nums[left] <= nums[mid] && nums[mid] >= nums[right] {
      left = mid + 1;
    } else if nums[mid] <= nums[right] && nums[mid] <= nums[left] {
      right = mid;
    } else {
      break;
    }

    // println!("{} {}", left, right);
  }

  // println!("{}", left);
  return left;
}

fn binary_search(arr: &[i32], mut left: usize, mut right: usize, value: i32) -> i32 {
  while left != right {
    let mid = (left + right) / 2;
    if arr[mid] < value {
      left = mid + 1;
    } else if value < arr[mid] {
      right = mid;
    } else {
      left = mid;
      break;
    }
  }

  if arr[left] == value {
    return left as i32;
  } else {
    return -1;
  }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
      let n = nums.len();
      let partition = find_partition(&nums);

      // println!("{}", partition);

      // Search from 0..left
      let index = if partition != 0 { binary_search(&nums, 0, partition - 1, target) } else { -1 };

      // Then search from left..n
      return if index != -1 { index } else { binary_search(&nums, partition, n - 1, target) };
    }
}
