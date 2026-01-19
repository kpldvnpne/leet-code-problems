pub struct Solution;

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

    #[test]
    fn test134() {
        let strs = vec!["xga", "xfb", "yfa"].iter().map(|it| it.to_string()).collect();
        let output = Solution::min_deletion_size(strs);
        assert_eq!(output, 1);
    }
}

fn get_ncols_to_del(strs: Vec<&[u8]>) -> i32 {
    let l = strs[0].len();
    let mut n_cols_to_del = 0;
    let mut prev_nested = vec![strs];
    for i in 0..l {
        // println!("{:?}", &prev_nested);
        let mut strictly_increasing = true;
        let mut deleted = false;
        let mut next_nested: Vec<Vec<&[u8]>> = vec![];
        let mut running = vec![];
        for list in &prev_nested {
            for str_i in 1..list.len() {
                let prev_str = list[str_i-1];
                let cur_str = list[str_i];
                if prev_str[i] > cur_str[i] {
                    n_cols_to_del += 1;
                    strictly_increasing = false;
                    deleted = true;
                    break; // Go to next index
                } else if prev_str[i] == cur_str[i] {
                    strictly_increasing = false;
                    if running.is_empty() {
                        running.push(prev_str);
                    } else if running[0][i] != prev_str[i] {
                        next_nested.push(running);
                        running = vec![prev_str];
                    }
                    running.push(cur_str);
                }
            }

            if deleted {
                break;
            }
        }

        if !running.is_empty() {
            next_nested.push(running);
        }

        if !deleted {
            prev_nested = next_nested;
        }

        if strictly_increasing {
            break;
        }

    }

    return n_cols_to_del;
}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs_as_bytes = strs.iter().map(|it| it.as_bytes()).collect();
        return get_ncols_to_del(strs_as_bytes);
    }
}
