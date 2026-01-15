pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_1() {
    let nums = vec![2,4,1,1,6,5];
    assert_eq!(Solution::count_hill_valley(nums), 3);
  }


  #[test]
  fn case_2() {
    let nums = vec![6,6,5,5,4,1];
    assert_eq!(Solution::count_hill_valley(nums), 0);
  }
}

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        // assert!(nums.len() >= 3); // Commented out to make the code faster
        let mut i = 0;
        let mut j = 2;
        let mut count = 0;

        while j < nums.len() {
          if nums[i] == nums[i+1] {
            i = i + 1;
            j = i + 2;
          } else if nums[j] == nums[j-1] {
            j = j + 1;
          } else {
            let left_diff = nums[i+1] - nums[i];
            let right_diff = nums[j] - nums[j-1];
            if left_diff * right_diff < 0 {
              count += 1;
            }

            i = j - 1;
            j = j + 1;
          }
        }

        return count;
    }
}
