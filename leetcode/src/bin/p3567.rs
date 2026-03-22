pub struct Solution;

fn main() {}

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let n_cols = n-k+1;
        let n_rows = m-k+1;
        let mut ans = vec![vec![0; n_cols]; n_rows];

        for i in 0..n_rows {
            for j in 0..n_cols {
                let mut v = Vec::with_capacity(k * k);
                for k_i in i..i+k {
                    for k_j in j..j+k {
                        v.push(grid[k_i][k_j]);
                    }
                }

                v.sort_unstable();
                let mut v_iter = v.iter();
                let mut prev = v_iter.next().unwrap();
                let mut min = i32::MAX;
                for cur in v_iter {
                    if cur == prev {
                        continue;
                    }
                    min = min.min(cur - prev);
                    prev = cur;
                }

                if min == i32::MAX {
                    ans[i][j] = 0;
                } else {
                    ans[i][j] = min;
                }
            }
        }

        return ans;
    }
}
