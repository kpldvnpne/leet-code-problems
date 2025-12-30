struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let input = [
      [4, 3, 8, 4],
      [9, 5, 1, 9],
      [2, 7, 6, 2],
    ].iter()
    .map(|i|
      i.iter().map(|i| {*i}).collect()
    )
    .collect();

    assert_eq!(Solution::num_magic_squares_inside(input), 1);
  }

  #[test]
  fn test_bitwise_set() {
    let mut set: u32 = 0;
    for i in 1..=9 {
      set |= 1 << (i-1);
    }

    assert_eq!(set, (1 << 9) - 1);
  }
}

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
       Solution::naive_solution(&grid)
    }

    fn naive_solution(grid: &Vec<Vec<i32>>) -> i32 {
      let n_rows = grid.len();
      let n_cols = grid[0].len();
      if n_rows < 3 || n_cols < 3 {
        return 0;
      }

      let mut n_magic_grids = 0;
      for row_start in 0..(n_rows - 2) {
        for col_start in 0..(n_cols - 2) {
          if Solution::is_magic_grid(grid, row_start, col_start) {
            n_magic_grids += 1;
          }
        }
      }
      return n_magic_grids;
    }

    fn is_magic_grid(grid: &Vec<Vec<i32>>, row_start: usize, col_start: usize) -> bool {
      let mut common_sum = -1;
      let mut set: u32 = 0;
      for row_i in row_start..row_start+3 {
        let mut row_sum = 0;
        for col_i in col_start..col_start+3 {
          if grid[row_i][col_i] > 9 {
            return false;
          }
          row_sum += grid[row_i][col_i];

          set |= 1 << (grid[row_i][col_i] - 1);
        }

        if common_sum == -1 {
          common_sum = row_sum;
        } else if common_sum != row_sum {
          return false;
        }
      }

      if set != ((1 << (10 - 1)) - 1) {
        return false;
      }

      for col_i in col_start..col_start+3 {
        let mut col_sum = 0;
        for row_i in row_start..row_start+3 {
          col_sum += grid[row_i][col_i];
        }

        if col_sum != common_sum {
          return false;
        }
      }

      let mut leading_diagonal_sum = 0;
      let mut non_leading_diagonal_sum = 0;
      for i in 0..3 {
        leading_diagonal_sum += grid[row_start+i][col_start+i];
        non_leading_diagonal_sum += grid[row_start+i][col_start+2-i];
      }
      if leading_diagonal_sum != common_sum || non_leading_diagonal_sum != common_sum {
        return false;
      }
      return true;
    }
}
