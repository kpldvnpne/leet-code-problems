pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let a = "11".to_string();
    let b = "1".to_string();

    let expected = "100".to_string();
    let actual = Solution::add_binary(a ,b);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let a = "1010".to_string();
    let b = "1011".to_string();

    let expected = "10101".to_string();
    let actual = Solution::add_binary(a ,b);

    assert_eq!(expected, actual);
  }
}

fn to_bit(c: char) -> u8 {
  c as u8 - '0' as u8
}

fn to_char(b: u8) -> char {
  (b + '0' as u8) as char
}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
      let mut new_string = String::new();
      let mut carry = 0;
      for (a_i, b_i) in a.chars().rev().zip(b.chars().rev()) {
        let a_i = to_bit(a_i);
        let b_i = to_bit(b_i);

        let sum = a_i + b_i + carry;

        if sum > 1 {
          carry = 1;
        } else {
          carry = 0;
        }

        new_string.push(to_char(sum & 1))
      }

      let rem_len = a.len().max(b.len()) - a.len().min(b.len());
      let rem_string: &str = match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Equal => &a[..rem_len],
        std::cmp::Ordering::Greater => &a[..rem_len],
        std::cmp::Ordering::Less => &b[..rem_len]
      };

      for r_i in rem_string.chars().rev() {
        let r_i = to_bit(r_i);
        let sum = r_i + carry;

        if sum > 1 {
          carry = 1;
        } else {
          carry = 0;
        }

        new_string.push(to_char(sum & 1));
      }

      if carry == 1 {
        new_string.push(to_char(1));
      }

      return new_string.chars().rev().collect();
    }
}
