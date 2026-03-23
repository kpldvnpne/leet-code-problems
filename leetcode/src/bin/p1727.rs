pub struct Solution;

fn main() {}

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut prev_heights = vec![0; n];
        let mut ans = 0;
        for i in 0..m {
          let mut heights = vec![0; n];
          for j in 0..n {
            if matrix[i][j] != 0 {
              heights[j] = prev_heights[j] + 1;
            }
          }

          prev_heights = heights.clone();
          heights.sort_unstable_by(|a, b| {b.cmp(&a)});

          for j in 0..n {
            if heights[j] == 0 {
              break;
            }
            let rect_size = (j + 1) * heights[j];
            ans = ans.max(rect_size);
          }
        }

        return ans as i32;
    }
}
