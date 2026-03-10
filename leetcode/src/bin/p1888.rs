pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let s = "111000".to_string();
    let expected = 2;
    let actual = Solution::min_flips(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s = "010".to_string();
    let expected = 0;
    let actual = Solution::min_flips(s);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let s = "1110".to_string();
    let expected = 1;
    let actual = Solution::min_flips(s);

    assert_eq!(expected, actual);
  }
}

impl Solution {
  pub fn min_flips(s: String) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut from_front = (vec![0; n], vec![0; n]);

    for (i, &c) in s.iter().enumerate() {
      if i > 0 {
        from_front.1[i] = from_front.0[i-1];
        from_front.0[i] = from_front.1[i-1];
      }
      if c == '0' as u8 {
        from_front.1[i] += 1;
      } else {
        from_front.0[i] += 1;
      }
    }

    if n % 2 == 0 {
      return from_front.1[n-1].min(from_front.0[n-1]);
    }

    let mut from_back = (vec![0; n], vec![0; n]);
    for (i, &c) in s.iter().enumerate().rev() {
      if i < n-1 {
        from_back.1[i] = from_back.0[i+1];
        from_back.0[i] = from_back.1[i+1];
      }

      if c == '0' as u8 {
        from_back.1[i] += 1;
      } else {
        from_back.0[i] += 1;
      }
    }

    let mut min_flips = from_back.0[0].min(from_back.1[0]);
    for i in 1..n {
      let for_start_0 = from_back.0[i] + from_front.0[i-1];

      let for_start_1 = from_back.1[i] + from_front.1[i-1];

      min_flips = min_flips.min(for_start_0.min(for_start_1));
    }

    return min_flips;
  }
}
