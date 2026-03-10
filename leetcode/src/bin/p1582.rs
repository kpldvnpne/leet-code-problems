pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let mat = vec![
      vec![1, 0, 0],
      vec![0, 0, 1],
      vec![1, 0, 0],
    ];

    let expected = 1;
    let actual = Solution::num_special(mat);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let mat = vec![
      vec![1, 0, 0],
      vec![0, 1, 0],
      vec![0, 0, 1],
    ];

    let expected = 3;
    let actual = Solution::num_special(mat);

    assert_eq!(expected, actual);
  }
}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut row_count = vec![0; m];
        let mut col_count = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    row_count[i] += 1;
                    col_count[j] += 1;
                }
            }
        }

        let rows_1 = row_count.into_iter().enumerate().filter(|it| it.1 == 1).map(|it| it.0);
        let cols_1 = col_count.into_iter().enumerate().filter(|it| it.1 == 1).map(|it| it.0).collect::<Vec<_>>();

        let mut ans = 0;
        for i in rows_1 {
            for &j in &cols_1 {
                if mat[i][j] == 1 {
                    ans += 1;
                }
            }
        }

        return ans;
    }
}
