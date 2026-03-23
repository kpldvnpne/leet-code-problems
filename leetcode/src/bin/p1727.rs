pub struct Solution;

fn main() {}


impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut prev_heights: Vec<(i32, usize)> = vec![];
        let mut ans = 0;
        for i in 0..m {
          let mut heights = Vec::with_capacity(n);
          let mut seen = vec![false; n];

          for (prev_height, j) in prev_heights {
            seen[j] = true;
            if matrix[i][j] == 1 {

              heights.push((prev_height + 1, j));
            }
          }

          for j in 0..n {
            if !seen[j] && matrix[i][j] == 1 {
              heights.push((1, j));
            }
          }

          for (j, &(height, _)) in heights.iter().enumerate() {
            ans = ans.max((j + 1) as i32 * height);
          }

          prev_heights = heights;
        }

        return ans;
    }
}
