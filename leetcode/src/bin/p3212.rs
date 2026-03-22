pub struct Solution;

fn main() {}

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let _m = grid.len();
        let n = grid[0].len();
        let mut prev_x_count = vec![0; n];
        let mut prev_y_count = vec![0; n];

        let mut ans = 0;
        for (_i, row) in grid.iter().enumerate() {
            let mut x_count = 0;
            let mut y_count = 0;
            for (j, &item) in row.iter().enumerate() {

                if item == 'X' {
                    x_count += 1;
                } else if item == 'Y' {
                    y_count += 1;
                }

                prev_x_count[j] += x_count;
                prev_y_count[j] += y_count;

                if prev_x_count[j] > 0 && prev_x_count[j] == prev_y_count[j] {
                    ans += 1;
                }
            }
        }

        return ans;
    }
}
