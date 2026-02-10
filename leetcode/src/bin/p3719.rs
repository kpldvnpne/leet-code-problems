pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = vec![2, 5, 4, 3];
    let expected = 4;
    let actual = Solution::longest_balanced(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = vec![3,2,2,5,4];
    let expected = 5;
    let actual = Solution::longest_balanced(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let nums = vec![1,2,3,2];
    let expected = 3;
    let actual = Solution::longest_balanced(nums);

    assert_eq!(expected, actual);
  }
}

impl Solution {
  pub fn longest_balanced(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    
    let mut longest_size = 0;
    use std::collections::HashSet;
    for i in 0..n {
      // Early exit
      let rem = n - i;
      if rem <= longest_size {
        break;
      }
      let mut evens = HashSet::new();
      let mut odds = HashSet::new();

      for j in i..n {
        let num = nums[j];
        if num % 2 == 0 {
          evens.insert(num);
        } else {
          odds.insert(num);
        }


        if evens.len() == odds.len() {
          longest_size = longest_size.max(j - i + 1);
        }
      }
    }
    
    return longest_size as i32;

  }
}