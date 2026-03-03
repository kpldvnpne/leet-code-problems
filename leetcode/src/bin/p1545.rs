pub struct Solution;

fn main() {}

fn invert(c: char) -> char {
    if c == '0' {
        return '1';
    } else {
        return '0';
    }
}

fn len_for(n: i32) -> i32 {
    let two_power_n = 1 << n;
    return two_power_n - 1;
}

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }

        // n > 1
        let len_for_below = len_for(n-1);
        let mid_point = len_for_below + 1;
        if k == mid_point {
            return '1';
        } else if k < mid_point { // k is valid so, > 0
            return Solution::find_kth_bit(n - 1, k);
        } else {
            // k > n // k is valid, so < 2n + 1
            return invert(Solution::find_kth_bit(n-1, len_for_below+1 - (k-mid_point)));
        }
    }
}
