// Problem: https://leetcode.com/problems/maximum-matrix-sum/description/

pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let matrix = vec![
            vec![1, -1],
            vec![-1, 1],
        ];

        let expected = 4;
        let actual = Solution::max_matrix_sum(matrix);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![-1, 2, 3],
            vec![1, 2, 3],
        ];

        let expected = 16;
        let actual = Solution::max_matrix_sum(matrix);

        assert_eq!(expected, actual);
    }
}

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut n_negatives = 0;
        let mut total_abs: i64 = 0;
        let mut min = i32::MAX;
        for row_i in 0..matrix.len() {
            for col_i in 0..matrix[0].len() {
                let item = matrix[row_i][col_i];
                if item < 0 {
                    n_negatives += 1;
                }
                let abs_item = i32::abs(item);
                min = i32::min(min, abs_item);
                total_abs += abs_item as i64;
            }
        }

        if n_negatives % 2 == 0 {
            return total_abs;
        } else {
            return total_abs - (2 * min) as i64;
        }
    }
}
