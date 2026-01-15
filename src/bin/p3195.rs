pub struct Solution;

use std::cmp::min;
use std::cmp::max;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        assert!(grid.len() >= 1, "Grid must be of len at least 1 for this to work");
        let mut top: i32 = -1;
        let mut bottom: i32 = -1;
        let mut left: i32 = -1;
        let mut right: i32 = -1;
        for row_i in 0..grid.len() {
            for col_i in 0..grid[0].len() {
                let item = grid[row_i][col_i];
                if item == 1 {
                    if top == -1 {
                        top = row_i as i32;
                    } else {
                        top = min(top, row_i as i32);
                    }

                    bottom = max(bottom, row_i as i32 + 1);

                    if left == -1 {
                        left = col_i as i32;
                    } else {
                        left = min(left, col_i as i32);
                    }

                    right = max(col_i as i32 + 1, right);
                }
            }
        }

        return (bottom - top) * (right - left);
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::minimum_area(vec![vec![0, 1, 0], vec![1, 0, 1]]), 6);
    }

    #[test]
    fn it_works_for_1_item() {
        assert_eq!(Solution::minimum_area(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]), 1);
    }
}
