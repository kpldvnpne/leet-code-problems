pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let arr = vec![4, 2, 1, 3];
    let expected = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
    let actual = Solution::minimum_abs_difference(arr);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let arr = vec![1, 3, 6, 10, 15];
    let expected = vec![vec![1, 3]];
    let actual = Solution::minimum_abs_difference(arr);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let arr = vec![3, 8, -10, 23, 19, -4, -14, 27];
    let expected = vec![vec![-14, -10], vec![19, 23], vec![23, 27]];
    let actual = Solution::minimum_abs_difference(arr);
    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn minimum_abs_difference(mut a: Vec<i32>) -> Vec<Vec<i32>> {
        a.sort_unstable();
        let n = a.len();
        let mut min_diff = i32::MAX;
        let mut pairs = vec![];
        for i in 0..n-1 {
            for j in i+1..n {
                let diff = (a[i] - a[j]).abs();
                if diff > min_diff {
                    break;
                }
                let pair = vec![a[i], a[j]];
                if diff == min_diff {
                    pairs.push(pair);
                } else if diff < min_diff {
                    pairs = vec![pair];
                    min_diff = diff;
                }
            }
        }

        return pairs;
    }
}
