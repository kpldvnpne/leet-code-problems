pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::Solution;

  #[test]
  fn test1() {
    let mat = vec![
      vec![1, 0, 1],
      vec![1, 1, 0],
      vec![1, 1, 0],
    ];

    let expected = 13;
    let actual = Solution::num_submat(mat);
    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let mat = vec![
      vec![0, 1, 1, 0],
      vec![0, 1, 1, 1],
      vec![1, 1, 1, 0],
    ];

    let expected = 24;
    let actual = Solution::num_submat(mat);
    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
      let mut row = vec![vec![0; mat[0].len()]; mat.len()];
      for i in 0..mat.len() {
        for j in 0..mat[i].len() {
          if mat[i][j] == 1 {
            if j == 0 {
              row[i][j] = 1;
            } else {
              row[i][j] = row[i][j-1] + 1;
            }
          }
        }
      }

      let mut n_submatrices = 0;
      for i in 0..row.len() {
        for j in 0..row[i].len() {
          let mut k = i;
          let mut row_sub = row[i][j];
          while row_sub > 0 {
            row_sub = std::cmp::min(row_sub, row[k][j]);
            n_submatrices += row_sub;


            if k == 0 {
              break;
            }

            k -= 1;
          }
        }
      }

      return n_submatrices;
    }
}
