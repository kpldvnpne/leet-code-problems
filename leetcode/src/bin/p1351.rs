pub struct Solution;

fn main() {
  let negs = Solution::count_negatives(
    vec![
      vec![4, 3, 2, -1],
      vec![3, 2, 1, -1],
      vec![1, 1, -1, -2],
      vec![-1, -1, -2, -3],
    ]
  );

  assert_eq!(negs, 8);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_1() {
    let negs = Solution::count_negatives(
      vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
      ]
    );

    assert_eq!(negs, 8);
  }
}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut total_negatives = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] < 0 {
                    total_negatives += 1;
                }
            }
        }

        return total_negatives;
    }
}
