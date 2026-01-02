struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test1() {
        let strs = vec!["ca", "bb", "ac"].iter().map(|it| it.to_string()).collect();
        let output = Solution::min_deletion_size(strs);
        assert_eq!(output, 1);
    }

    #[test]
    fn test2() {
        let strs = vec!["xc", "yb", "za"].iter().map(|it| it.to_string()).collect();
        let output = Solution::min_deletion_size(strs);
        assert_eq!(output, 0);
    }

    #[test]
    fn test3() {
        let strs = vec!["xc", "yb", "za"].iter().map(|it| it.to_string()).collect();
        let output = Solution::min_deletion_size(strs);
        assert_eq!(output, 0);
    }
}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let l = strs[0].len();
        let mut n_cols_delete = 0;
        for col in 0..l {
            let mut col_deleted = false;
            for row in 1..strs.len() {
                let cur_str = &strs[row];
                let prev_str = &strs[row-1];

                let cur_str_char = cur_str.chars().nth(col).unwrap();
                let prev_str_char = prev_str.chars().nth(col).unwrap();
                if cur_str_char < prev_str_char {
                    n_cols_delete += 1;
                    col_deleted = true;
                } else if cur_str_char == prev_str_char {
                    
                }
            }
            if !col_deleted {
                break;
            }
        }

        return n_cols_delete;
    }
}