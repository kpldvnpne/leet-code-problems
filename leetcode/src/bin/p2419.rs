pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_1() {
    let nums = vec![1, 2, 3, 3, 2, 2];
    assert_eq!(Solution::longest_subarray(nums), 2);
  }

  #[test]
  fn case_2() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::longest_subarray(nums), 1);
  }

  #[test]
  fn case_3() {
    let nums = vec![1, 4, 4, 4, 3, 4, 4];
    assert_eq!(Solution::longest_subarray(nums), 3);
  }
}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
      assert!(nums.len() > 0, "Nums should have at least 1 member");

      let mut max_num = &nums[0];
      let mut max_num_max_len = 1;
      let mut max_num_cur_len = 1;

      for num in nums.iter().skip(1) {
        if num == max_num {
          max_num_cur_len += 1;
          if max_num_cur_len > max_num_max_len {
            max_num_max_len = max_num_cur_len;
          }
        } else if num > max_num {
          max_num = num;
          max_num_cur_len = 1;
          max_num_max_len = 1;
        } else {
          max_num_cur_len = 0;
        }
      }

      return max_num_max_len;
    }
}

