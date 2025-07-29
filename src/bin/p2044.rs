pub struct Solution;

fn main() {}

fn bitwise_or(nums: &Vec<i32>) -> i32 {
  assert!(nums.len() > 0);
  let mut acc = 0;
  for num in nums {
    acc = acc | num;
  }
  return acc;
}

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
      let max_bitwise_or: i32 = bitwise_or(&nums);
      let mut answer: i32 = 0;

      for mask in 0..(2i32.pow(nums.len() as u32)) {
        let mut bitwise_or = 0;
        for num_i in 0..nums.len() {
          let should_take = (mask & (1 << num_i)) >> num_i;
          if should_take == 1 {
            bitwise_or = bitwise_or | nums[num_i];
          }
        }

        if bitwise_or == max_bitwise_or {
          answer += 1;
        }
      }

      return answer;
    }

}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_works() {
    let nums = vec![3, 1];
    assert_eq!(Solution::count_max_or_subsets(nums), 2);
    let nums = vec![2, 2, 2];
    assert_eq!(Solution::count_max_or_subsets(nums), 7);
    let nums = vec![3, 2, 1, 5];
    assert_eq!(Solution::count_max_or_subsets(nums), 6);
  }
}
