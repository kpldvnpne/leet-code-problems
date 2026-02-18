pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let s = "abbac".to_string();
    let expected = 4;
    let actual = Solution::longest_balanced(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s = "zzabccy".to_string();
    let expected = 4;
    let actual = Solution::longest_balanced(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let s = "aba".to_string();
    let expected = 2;
    let actual = Solution::longest_balanced(s);

    assert_eq!(expected, actual);
  }
}

fn to_index(c: char) -> usize {
  c as usize - 'a' as usize
}

fn is_diff_balanced(c1: &[i32], c2: &[i32]) -> bool {
  let mut i = 0;
  let mut last = -1;
  while i < c1.len() {
    let diff = c2[i] - c1[i];
    if diff > 0 {
      if last == -1 {
        last = diff;
      } else {
        if diff != last {
          return false;
        }
      }
    }
    i = i + 1;
  }

  return true;
}

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
      let mut count = [0; 26];
      let mut counts = Vec::with_capacity(s.len() + 1);
      counts.push(count.clone());
      for c in s.chars() {
        let index = to_index(c);
        count[index] += 1;
        counts.push(count.clone());
      }

      for len in (1..=s.len()).rev() {
        for start in 0..=(s.len() - len) {
          let end = start + len;
          if is_diff_balanced(&counts[start], &counts[end]) {
            return len as i32;
          }
        }
      }

      unreachable!();
    }
}
