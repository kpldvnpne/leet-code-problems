struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  fn into_char_matrix<T, I>(matrix: &[T]) -> Vec<Vec<char>>
    where
      T: AsRef<[I]>,
      I: std::fmt::Display,
  {
    matrix
      .iter()
      .map(|it|
        it.as_ref().iter()
        .map(|i| format!("{}", i).chars().next().unwrap())
        .collect())
      .collect()
  }

  #[test]
  fn test1() {
    let matrix = [
      [1, 0, 1, 0, 0],
      [1, 0, 1, 1, 1],
      [1, 1, 1, 1 ,1],
      [1, 0, 0, 1, 0],
    ];

    let matrix = into_char_matrix(&matrix);

    let expected = 6;
    let actual = Solution::maximal_rectangle(matrix);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let matrix = [
      [0]
    ];
    let matrix = into_char_matrix(&matrix);

    let expected = 0;
    let actual = Solution::maximal_rectangle(matrix);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let matrix = [
      [1]
    ];
    let matrix = into_char_matrix(&matrix);

    let expected = 1;
    let actual = Solution::maximal_rectangle(matrix);

    assert_eq!(expected, actual);
  }


}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
      let n_rows = matrix.len();
      let n_cols = matrix[0].len();
      let mut row_runs = vec![vec![0; n_cols]; n_rows];

      for i in 0..n_rows {
        let mut run = 0;
        for j in 0..n_cols {
          if matrix[i][j] == '1' {
            run += 1;
          } else {
            run = 0;
          }

          row_runs[i][j] = run;
        }
      }

      let mut max_rect = 0;
      for i in 0..n_rows {
        for j in 0..n_cols {
          let mut cur_row_width = row_runs[i][j];
          let mut cur_max_rect = cur_row_width;
          let mut k = i;
          while cur_row_width > 0 && k < n_rows {
            cur_row_width = std::cmp::min(cur_row_width, row_runs[k][j]);
            let cur_row_rect = cur_row_width * (k - i + 1);
            cur_max_rect = std::cmp::max(cur_max_rect, cur_row_rect);

            k = k + 1;
          }

          // println!("Found {} at {} {}", cur_max_rect, i, j);

          max_rect = std::cmp::max(max_rect, cur_max_rect);
        }
      }

      return max_rect as i32;
    }
}
