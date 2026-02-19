pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let s = "00110011".to_string();
    let expected = 6;
    let actual = Solution::count_binary_substrings(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s = "10101".to_string();
    let expected = 4;
    let actual = Solution::count_binary_substrings(s);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
      let mut last_count = 0;
      let mut cur_count = 1;
      let mut last_char = s.chars().next().unwrap();

      let mut ans = 0;

      for c in s.chars().skip(1) {
        if c == last_char {
          cur_count += 1;
        } else {
          ans += last_count.min(cur_count);
          last_count = cur_count;
          cur_count = 1;
        }
        last_char = c;
      }
      ans += last_count.min(cur_count);

      return ans;
    }
}
