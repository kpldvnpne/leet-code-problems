pub struct Solution;

fn main() {}

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut suffix_sum = vec![vec![0; n]; m];

        for j in 0..n {
            let mut sum = 0;
            for i in (0..m).rev() {
                if matrix[i][j] == 0 {
                    sum = 0;
                } else {
                    sum += matrix[i][j];
                }
                suffix_sum[i][j] = sum;
            }
        }

        let mut ans = 0;
        for i in 0..m {
            suffix_sum[i].sort_unstable_by(|a, b| { b.cmp(&a) });
            for j in 0..n {
                if suffix_sum[i][j] == 0 {
                    break;
                }
                let rect = (j + 1) as i32 * suffix_sum[i][j];
                ans = ans.max(rect);
            }
        }

        return ans;
    }
}
