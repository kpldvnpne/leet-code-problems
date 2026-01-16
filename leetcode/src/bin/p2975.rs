pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let m = 4;
        let n = 3;
        let h_fences = vec![2, 3];
        let v_fences = vec![2];

        let expected = 4;
        let actual = Solution::maximize_square_area(m, n, h_fences, v_fences);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let m = 6;
        let n = 7;
        let h_fences = vec![2];
        let v_fences = vec![4];

        let expected = -1;
        let actual = Solution::maximize_square_area(m, n, h_fences, v_fences);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let m = 3;
        let n = 9;
        let h_fences = vec![2];
        let v_fences = vec![8, 6, 5, 4];

        let expected = 4;
        let actual = Solution::maximize_square_area(m, n, h_fences, v_fences);

        assert_eq!(expected, actual);
    }
}

use std::collections::HashSet;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        let mut h = vec![1];
        for f in h_fences {
            h.push(f);
        }
        h.push(m);
        h.sort();

        let mut v = vec![1];
        for f in v_fences {
            v.push(f);
        }
        v.push(n);
        v.sort();

        let (small, big) = if h.len() < v.len() { (&h, &v) } else { (&v, &h) };

        let mut small_intervals = HashSet::new();
        for i in 0..(small.len() - 1) {
            for j in i..(small.len()) {
                let interval = small[j] - small[i];
                small_intervals.insert(interval);
            }
        }

        let mut max_common_interval = -1;
        for i in 0..(big.len() - 1) {
            let max_possible_interval = big[big.len() - 1] - big[i];
            if max_possible_interval <= max_common_interval {
                break;
            }
            for j in ((i + 1)..(big.len())).rev() {
                let interval = big[j] - big[i];
                if small_intervals.contains(&interval) {
                    max_common_interval = std::cmp::max(interval, max_common_interval);
                    break;
                }
            }
        }

        let ans = if max_common_interval == -1 { -1 } else { max_common_interval as i64 * max_common_interval as i64 };

        return (ans % (1_000_000_000 + 7)) as i32;
    }
}
