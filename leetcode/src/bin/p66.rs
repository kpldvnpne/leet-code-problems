pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let digits = vec![1, 2, 3];
    let expected = vec![1, 2, 4];
    let actual = Solution::plus_one(digits);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let digits = vec![4, 3, 2, 1];
    let expected = vec![4, 3, 2, 2];
    let actual = Solution::plus_one(digits);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let digits = vec![9];
    let expected = vec![1, 0];
    let actual = Solution::plus_one(digits);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        for index in (0..digits.len()).rev() {
          let mut new_digit = digits[index] + carry;
          if new_digit > 9 {
            new_digit = new_digit - 10;
            carry = 1;
          } else {
            carry = 0;
          }
          digits[index] = new_digit;
        }
        if carry != 0 {
            let mut new_digits = vec![carry];
            new_digits.append(&mut digits);
            return new_digits;
        } else {
            return digits;
        }
    }
}
