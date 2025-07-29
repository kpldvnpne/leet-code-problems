pub struct Solution;

fn main() {}

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
      let mut answers: Vec<i32> = vec![0; nums.len()];
      let mut max_bitwise_ors = vec![0; nums.len()];

      let mut bitwise_or = 0;
      for (num_i, num) in nums.iter().enumerate().rev() {
        bitwise_or = bitwise_or | num;
        max_bitwise_ors[num_i] = bitwise_or;
      }

      for start_i in 0..(nums.len()) {
        let mut bitwise_or = nums[start_i];
        let mut j = start_i + 1;
        while j < nums.len() && bitwise_or < max_bitwise_ors[start_i] {
          bitwise_or = bitwise_or | nums[j];
          j = j + 1;
        }

        let answer = j - start_i;
        answers[start_i] = answer as i32;
      }

      return answers;
    }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn the_vec_dynamic_init_works() {
    let mut len = 0;
    len += 5;
    let a_list = vec![0; len];

    assert_eq!(a_list.len(), 5);
  }

  #[test]
  fn it_works_for_case_1() {
    let nums = vec![1, 0, 2, 1, 3];
    let answer = vec![3, 3, 2, 2, 1];

    assert_eq!(Solution::smallest_subarrays(nums), answer);
  }

  #[test]
  fn it_works_for_case_2() {
    let nums = vec![1, 2];
    let answer = vec![2, 1];

    assert_eq!(Solution::smallest_subarrays(nums), answer);
  }

  #[test]
  fn it_works_when_last_element_is_zero() {
    let nums = vec![1, 0];
    let answer = vec![1, 1];

    assert_eq!(Solution::smallest_subarrays(nums), answer);
  }
}
