pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let squares = vec![
      vec![0, 0, 1],
      vec![2, 2 ,1],
    ];

    let expected = 1.00000;
    let actual = Solution::separate_squares(squares);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let squares = vec![
      vec![0, 0, 2],
      vec![1, 1, 1],
    ];

    let expected = 1.00000;
    let actual = Solution::separate_squares(squares);

    assert_eq!(expected, actual);
  }
}

use std::collections::BTreeSet;

struct IntervalTree {
  xs: Vec<i32>,
  
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
      let mut unique_xs = BTreeSet::new();
      for square in squares {
        let x = square[0];
        let y = square[1];
        let l = square[2];

        unique_xs.insert(x);
        unique_xs.insert(x + l);
      }

      return _;
    }
}
