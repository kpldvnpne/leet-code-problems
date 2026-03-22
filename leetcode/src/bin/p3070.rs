pub struct Solution;

fn main() {}

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut prev_sums = vec![0; n];

        let mut max_j = n;
        let mut ans = 0;
        for i in 0..m {
            let mut cur_row_sum = 0;
            for j in 0..max_j {
                cur_row_sum += grid[i][j];
                let sum = cur_row_sum + prev_sums[j];
                prev_sums[j] = sum;

                if sum <= k {
                    ans += 1;
                } else {
                    max_j = j;
                    break;
                }
            }

            if prev_sums[0] > k {
                break;
            }
        }

        return ans;
    }
}
