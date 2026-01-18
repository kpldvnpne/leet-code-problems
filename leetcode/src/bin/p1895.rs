pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  trait ToVecOfVec<T> {
    fn to_vec_of_vec(self) -> Vec<Vec<T>>;
  }

  impl<I, J, T> ToVecOfVec<T> for I
    where I: IntoIterator<Item=J>, J: IntoIterator<Item = T>
  {
    fn to_vec_of_vec(self) -> Vec<Vec<T>> {
      self.into_iter()
        .map(|inner| inner.into_iter().collect())
        .collect()
    }
  }

  #[test]
  fn test1() {
    let grid = [
      [7,1,4,5,6],
      [2,5,1,6,4],
      [1,5,4,3,2],
      [1,2,7,3,4]
    ].to_vec_of_vec();
    let expected = 3;
    let actual = Solution::largest_magic_square(grid);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let grid = [[5,1,3,1],[9,3,3,1],[1,3,3,8]].to_vec_of_vec();
    let expected = 2;
    let actual = Solution::largest_magic_square(grid);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test80() {
    let grid = [[1,3,1],[1,1,3],[3,1,3]].to_vec_of_vec();
    let expected = 1;
    let actual = Solution::largest_magic_square(grid);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
      let m = grid.len();
      let n = grid[0].len();
      let mut row_sums = vec![vec![0; n]; m];

      for i in 0..m {
        row_sums[i][0] = grid[i][0];
        for j in 1..n {
          row_sums[i][j] = row_sums[i][j-1] + grid[i][j];
        }
      }

      let mut col_sums = vec![vec![0; n]; m];
      for j in 0..n {
        col_sums[0][j] = grid[0][j];
        for i in 1..m {
          col_sums[i][j] = col_sums[i-1][j] + grid[i][j];
        }
      }

      let mut lead_diag_sums = vec![vec![0; n]; m];
      for col_start in (0..n).rev() {
        lead_diag_sums[0][col_start] = grid[0][col_start];
        let mut step = 1;
        while col_start + step < n && step < m {
          let i = step;
          let j = col_start+step;
          lead_diag_sums[i][j] = lead_diag_sums[i-1][j-1] + grid[i][j];

          step = step + 1;
        }
      }

      for row_start in 1..m {
        lead_diag_sums[row_start][0] = grid[row_start][0];
        let mut i = row_start + 1;
        let mut j = 1;
        while i < m && j < n {
          lead_diag_sums[i][j] = lead_diag_sums[i-1][j-1] + grid[i][j];
          i = i + 1;
          j = j + 1;
        }
      }

      let mut rev_diag_sums = vec![vec![0; n]; m];
      for col_start in 0..n {
        rev_diag_sums[0][col_start] = grid[0][col_start];
        let mut i = 1;
        let mut j: i32 = col_start as i32 - 1;
        while i < m && j >= 0 {
          rev_diag_sums[i][j as usize] = rev_diag_sums[i-1][j as usize +1] + grid[i][j as usize];
          i = i + 1;
          j = j - 1;
        }
      }

      for row_start in 1..m {
        let mut i = row_start;
        let mut j: i32 = n as i32 - 1;
        rev_diag_sums[i][j as usize] = grid[i][j as usize];

        i = i + 1;
        j = j - 1;
        while i < m && j >= 0 {
          rev_diag_sums[i][j as usize] = rev_diag_sums[i-1][j as usize +1] + grid[i][j as usize];
          i = i + 1;
          j = j - 1;
        }
      }

      // fn print_2d(v: &Vec<Vec<i32>>) {
      //   println!("---");
      //   for r in v {
      //     println!("{:?}", r);
      //   }
      // }

      // print_2d(&row_sums);
      // print_2d(&col_sums);
      // print_2d(&lead_diag_sums);
      // print_2d(&rev_diag_sums);

      let mut max_sq_size = 1;
      'sq_size: for sq_size in (2..=std::cmp::min(m,n)).rev() {
        for row in (sq_size-1)..m {
          'next_col: for col in (sq_size-1)..n {
            let lead_diag_sum = if row >= sq_size && col >= sq_size {
              lead_diag_sums[row][col] - lead_diag_sums[row-sq_size][col-sq_size]
            } else {
              lead_diag_sums[row][col]
            };

            let rev_diag_sum = if row >= sq_size && col < n - 1{
              rev_diag_sums[row][col-(sq_size-1)] - rev_diag_sums[row-(sq_size)][col+1]
            } else {
              rev_diag_sums[row][col-(sq_size-1)]
            };

            // println!("Diag sums: {} {}", lead_diag_sum, rev_diag_sum);

            if lead_diag_sum != rev_diag_sum {
              // println!("Diag sum don't match {row} {col}");
              continue;
            }

            let req_sum = lead_diag_sum;

            // Check all rows
            for row_i in (row-(sq_size-1))..=row {
              let row_sum = if col >= sq_size {
                row_sums[row_i][col] - row_sums[row_i][col-sq_size]
              } else {
                row_sums[row_i][col]
              };

              if row_sum != req_sum {
                continue 'next_col;
              }
            }

            for col_i in (col-(sq_size-1))..=col {
              let col_sum = if row >= sq_size {
                col_sums[row][col_i] - col_sums[row-sq_size][col_i]
              } else {
                col_sums[row][col_i]
              };

              if col_sum != req_sum {
                continue 'next_col;
              }
            }

            // println!("Found at {} {} with size {}", row, col, sq_size);
            max_sq_size = sq_size;
            break 'sq_size;
          }
        }
      }

      return max_sq_size as i32;
    }
}
