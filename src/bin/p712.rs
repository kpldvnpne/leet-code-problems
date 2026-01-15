pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let s1 = "sea".to_string();
        let s2 = "eat".to_string();

        let expected = 231;
        let actual = Solution::minimum_delete_sum(s1, s2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let s1 = "delete".to_string();
        let s2 = "leet".to_string();

        let expected = 403;
        let actual = Solution::minimum_delete_sum(s1, s2);

        assert_eq!(expected, actual);
    }
}

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        let n_rows = s1.len();
        let n_cols = s2.len();

        let mut dp = vec![vec![0; n_cols]; n_rows];

        for i in (0..n_rows).rev() {
            for j in (0..n_cols).rev() {
                let cur_ascii = if s1[i] == s2[j] {
                    2 * s1[i] as i32
                } else {
                    0
                };
                let right = if j < (n_cols - 1) { dp[i][j+1] } else { 0 };
                let down = if i < (n_rows - 1) { dp[i+1][j] } else { 0 };
                let right_down = if i < (n_rows-1) && j < (n_cols-1) { dp[i+1][j+1] } else { 0 };
                dp[i][j] = *vec![right, down, cur_ascii + right_down].iter().max().unwrap();
            }
        }

        // Return all delete - max for min
        let mut all_delete = 0;
        for item in s1 {
            all_delete += *item as i32;
        }
        for item in s2 {
            all_delete += *item as i32;
        }
        let min_delete = all_delete - dp[0][0];
        return min_delete;
    }
}
