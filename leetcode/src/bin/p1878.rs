pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let grid = [[3,4,5,1,3],[3,3,4,2,3],[20,30,200,40,10],[1,5,5,4,1],[4,3,2,2,5]].map(|it| it.into_iter().collect::<Vec<_>>()).into_iter().collect::<Vec<_>>();
    let expected = vec![228,216,211];
    let actual = Solution::get_biggest_three(grid);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let grid = [[1,2,3],[4,5,6],[7,8,9]].map(|it| it.into_iter().collect::<Vec<_>>()).into_iter().collect::<Vec<_>>();
    let expected = vec![20,9,8];
    let actual = Solution::get_biggest_three(grid);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let grid = [[7,7,7]].map(|it| it.into_iter().collect::<Vec<_>>()).into_iter().collect::<Vec<_>>();
    let expected = vec![7];
    let actual = Solution::get_biggest_three(grid);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut leading_sum = vec![vec![0; n]; m];
        let mut non_leading_sum = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                let prev = if i > 0 && j > 0 { leading_sum[i-1][j-1] } else { 0 };
                leading_sum[i][j] = prev + grid[i][j];

                let prev = if i > 0 && j < n-1 { non_leading_sum[i-1][j+1] } else { 0 };
                non_leading_sum[i][j] = prev + grid[i][j];
            }
        }

        use std::collections::BTreeSet;
        let mut ans = BTreeSet::new();
        for i in 0..m {
            for j in 0..n {
                let mut sum = grid[i][j];
                ans.insert(sum);
                if ans.len() > 3 {
                  ans.pop_first();
                }

                let left_indices = (i+1..m).zip((0..j).rev());
                let right_indices = (i+1..m).zip(j+1..n);
                let bottom_indices = (i+2..m).step_by(2).map(|i| {(i, j)});

                for ((left_index, right_index), bottom_index) in left_indices.zip(right_indices).zip(bottom_indices) {
                    sum += grid[left_index.0][left_index.1];
                    sum += grid[right_index.0][right_index.1];

                    let left_side_sum = leading_sum[bottom_index.0][bottom_index.1] - leading_sum[left_index.0][left_index.1];
                    let right_side_sum = non_leading_sum[bottom_index.0][bottom_index.1] - non_leading_sum[right_index.0][right_index.1];
                    let this_rhombus_sum = sum + left_side_sum + right_side_sum - grid[bottom_index.0][bottom_index.1];

                    ans.insert(this_rhombus_sum);
                    if ans.len() > 3 {
                      ans.pop_first();
                    }
                }
            }
        }

        return ans.into_iter().rev().collect();
    }
}
