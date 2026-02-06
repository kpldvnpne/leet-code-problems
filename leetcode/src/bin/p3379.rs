pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![3, -2, 1, 1];
    let expected = vec![1, 1, 1, 3];
    let actual = Solution::construct_transformed_array(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![-10, -10, -4];
    let expected = vec![-4, -10, -10];
    let actual = Solution::construct_transformed_array(nums);

    assert_eq!(expected, actual);
  }

  // #[test]
  // fn test_mod() {
  //   println!("{}", -4 % 5);
  // }
}

impl Solution {
  pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0; nums.len()];
    for i in 0..nums.len() {
      let new_index = ((nums[i] + i as i32) as i32 % nums.len() as i32 + nums.len() as i32) as usize % nums.len();
      ans[i] = nums[new_index];
    }

    return ans;
  }
}