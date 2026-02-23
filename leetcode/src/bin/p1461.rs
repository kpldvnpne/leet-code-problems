pub struct Solution;

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let s = "00110110".to_string();
    let k = 2;
    let expected = true;
    let actual = Solution::has_all_codes(s, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let s = "0110".to_string();
    let k = 1;
    let expected = true;
    let actual = Solution::has_all_codes(s, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let s = "0110".to_string();
    let k = 2;
    let expected = false;
    let actual = Solution::has_all_codes(s, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test4() {
    let s = "101000100011111001110010".to_string();
    let k = 3;
    let expected = true;
    let actual = Solution::has_all_codes(s, k);

    assert_eq!(expected, actual);
  }
}

fn main() {}

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
      if k as usize > s.len() {
        return false;
      }
      let len = 1 << k;
      let mut presence = vec![false; len];

      let s = s.as_bytes();

      let mut cur: usize = 0;
      let k = k as usize;
      for i in 0..(k-1) {
        let bit = s[i] - '0' as u8;
        // println!("Bit: {}", bit);
        cur = (cur << 1) + bit as usize;
        // println!("{}", cur);
      }

      // println!("Initial: {}", cur);

      let mask = (1 << k) - 1;
      // println!("{}", mask);
      for i in (k-1)..s.len() {
        let bit = s[i] - '0' as u8;
        cur = ((cur << 1) & mask) | bit as usize;

        // println!("{}", cur);

        presence[cur] = true;
      }

      // println!("{:?}", presence);

      let high = len;
      for n in 0..high {
        if !presence[n] {
          return false;
        }
      }

      return true;
    }
}
