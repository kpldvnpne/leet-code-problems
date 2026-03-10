pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let nums = ["01","10"].iter().map(|it| it.to_string()).collect::<Vec<_>>();
    let expected = "00";
    let actual = Solution::find_different_binary_string(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let nums = ["00","01"].iter().map(|it| it.to_string()).collect::<Vec<_>>();
    let expected = "10";
    let actual = Solution::find_different_binary_string(nums);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let nums = ["111","011", "001"].iter().map(|it| it.to_string()).collect::<Vec<_>>();
    let expected = "000";
    let actual = Solution::find_different_binary_string(nums);

    assert_eq!(expected, actual);
  }
}

fn to_i32(bin_s: &str) -> i32 {
  let mut result = 0;
  for c in bin_s.chars() {
    let bit = c as i32 - '0' as i32;
    // println!("{}", bit);
    result = (result << 1) + bit;
  }
  return result;
}

fn to_str(num: i32, min_size: usize) -> String {
  let mut ans = String::new();
  for i in (0..min_size).rev() {
    let bit = (num >> i) & 1;
    let bit_c = (bit as u8 + '0' as u8) as char;
    ans.push(bit_c);
  }
  return ans;
}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
      let n = nums.len();
      use std::collections::HashSet;
      let mut set = HashSet::new();
      for num in nums {
        let num_i32 = to_i32(&num);
        // println!("{}", num_i32);
        set.insert(num_i32);
      }

      // println!("{:?}", set);

      let max_num = 1 << n;
      for i in 0..max_num {
        if !set.contains(&i) {
          return to_str(i, n);
        }
      }

      unreachable!();
    }
}
