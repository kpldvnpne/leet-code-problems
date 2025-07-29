pub struct Solution;

fn main() {}

// TODO: Didn't beat everybody
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
      return Solution::method_2(&nums);
    }

    fn method_2(nums: &Vec<i32>) -> i32 {
      let max_bitwise_or = bitwise_or(nums);

      return Solution::dfs(&0, nums, 0, &max_bitwise_or);
    }

    // It is faster because we are caching bitwise_or
    // It could be made faster with stack instead of recursion.
    fn dfs(bitwise_or: &i32, nums: &Vec<i32>, index: usize, max_bitwise_or: &i32) -> i32 {
      if index == nums.len() {
        if bitwise_or == max_bitwise_or {
          return 1;
        } else {
          return 0;
        }
      }

      return Solution::dfs(&(bitwise_or | nums[index]), nums, index + 1, max_bitwise_or)
        + Solution::dfs(bitwise_or, nums, index + 1, max_bitwise_or) ;
    }

    #[allow(dead_code)]
    fn method_1(nums: &Vec<i32>) -> i32 {
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
