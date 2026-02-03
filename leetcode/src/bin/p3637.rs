pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let nums = vec![1, 3, 5, 4, 2, 6];
    let expected = true;
    let actual = Solution::is_trionic(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![2, 1, 3];
    let expected = false;
    let actual = Solution::is_trionic(nums);

    assert_eq!(expected, actual);
  }
}

enum Status {
  Increasing,
  Decreasing,
}

impl Solution {
  pub fn is_trionic(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if nums[0] >= nums[1] {
      return false;
    }
    let mut status = Status::Increasing;
    let mut n_changed = 1;
    for i in 2..n {
      if nums[i-1] == nums[i] {
        return false;
      }
      if matches!(status, Status::Increasing) {
        if nums[i-1] > nums[i] {
          status = Status::Decreasing;
          n_changed += 1;
        }
      } else { // Decreasing
        if nums[i-1] < nums[i] {
          status = Status::Increasing;
          n_changed += 1;
        }
      }

      if n_changed > 3 {
        return false;
      }
    } 
    return n_changed == 3;
  }

  pub fn is_trionic_v1(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n < 4 {
      return false;
    }

    let start_p_i = 1;
    let mut p_i = start_p_i;
    while p_i < n && nums[p_i-1] < nums[p_i] {
      p_i = p_i + 1;
    }

    if p_i == start_p_i {
      return false;
    }

    let start_q_i = p_i;
    let mut q_i = start_q_i;
    while q_i < n && nums[q_i-1] > nums[q_i] {
      q_i = q_i + 1;
    }

    if q_i == start_q_i {
      return false;
    }

    let start_r_i = q_i;
    let mut r_i = start_r_i;
    while r_i < n && nums[r_i-1] < nums[r_i] {
      r_i = r_i + 1;
    }

    if r_i == start_r_i || r_i != n {
      return false;
    }

    return true;
  }
}
