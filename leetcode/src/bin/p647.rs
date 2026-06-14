pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let s = "abc".to_string();
    let expected = 3;
    let actual = Solution::count_substrings(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s = "aaa".to_string();
    let expected = 6;
    let actual = Solution::count_substrings(s);

    assert_eq!(expected, actual);
  }
}

impl Solution {
  pub fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let mut n_palindromes = 0;
    let n = s.len();

    for i in 0..n {
      // The character itself
      n_palindromes += 1;

      // Odd-len palindromes
      let mut left = i;
      let mut right = i;
      while left > 0 && right < n - 1 {
        left -= 1;
        right += 1;

        if s[left] != s[right] {
          break;
        }

        n_palindromes += 1;
      }

      // Even-len palindromes
      let mut left = i + 1;
      let mut right = i;
      while left > 0 && right < n - 1 {
        left -= 1;
        right += 1;

        if s[left] != s[right] {
          break;
        }

        n_palindromes += 1;
      }
    }

    return n_palindromes;
  }
}
