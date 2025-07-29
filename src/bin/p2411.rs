pub struct Solution;

fn main() {}

// TODO: This is currently not efficient
impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
      return Solution::method_2_sliding_window(&nums);
    }

    fn method_2_sliding_window(nums: &Vec<i32>) -> Vec<i32> {
      let mut max_bitwise_ors = vec![0; nums.len()];

      let mut bitwise_or = 0;
      for (num_i, num) in nums.iter().enumerate().rev() {
        bitwise_or = bitwise_or | num;
        max_bitwise_ors[num_i] = bitwise_or;
      }

      let mut residues = nums.clone();
      let mut residue_update_start: usize = 0; // Once 0, no need to keep updating
      let mut answers: Vec<i32> = vec![0; nums.len()];
      let mut answer_i: usize = 0;
      let mut rolling_bitwise_or = 0;

      for (num_i, num) in nums.iter().enumerate() {
        rolling_bitwise_or = rolling_bitwise_or | num;
        for residue_i in residue_update_start..num_i {
          let residue = residues[residue_i];
          let new_residue = residue ^ (residue & num);
          residues[residue_i] = new_residue;

          // If 0, no need to keep updating
          if new_residue == 0 {
            residue_update_start += 1;
          }
        }

        while (answer_i <= num_i) && (rolling_bitwise_or == max_bitwise_ors[answer_i])  {
          let answer = (num_i - answer_i + 1) as i32;
          answers[answer_i] = answer;
          rolling_bitwise_or = rolling_bitwise_or ^ residues[answer_i];
          answer_i += 1;
        }
      }

      return answers;
    }

    #[allow(dead_code)]
    fn method_1_slow(nums: &Vec<i32>) -> Vec<i32> {
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

  #[test]
  fn it_works_for_a_failing_test() {
    let nums = vec![76,57,77,64,41,82,34,57,94,33,87,24,17,54,11,10,18,70,80,37,18,5,11,71,0,29,90,45,25,64,64,32,3,67,78,4,98,2,50,100,9,97,48,47,23,28,25,45,56,37,30,72,57,58,39,89,78,13,76,40,44,79,41,99,24,7,86,23,42,58,34,14,11,26];
    let expected_answer = vec![6,5,4,6,5,4,3,2,2,3,4,7,6,5,6,5,7,6,5,5,8,7,6,5,4,3,2,6,7,10,9,8,7,6,7,6,5,4,3,5,4,3,10,9,8,7,6,5,4,3,3,4,4,3,2,5,9,8,7,6,5,4,4,3,5,4,3,2,5,4,4,3,2,1];
    let real_answer = Solution::smallest_subarrays(nums);

    assert_eq!(real_answer, expected_answer);
  }

  fn read_vec_from_file(path: &str) -> Vec<i32> {
    use std::fs;

    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let vec: Vec<i32> = contents.trim().split(",").map(|it| {
      it.parse::<i32>().unwrap()
    }).collect();

    return vec;
  }

  #[test]
  fn it_works_for_large_sized_nums() {
    let nums = read_vec_from_file("/Users/kapildev/Documents/rust-leetcode/src/bin/p2411_large_input.txt");

    Solution::smallest_subarrays(nums);
  }

}
