pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![2, 3, 5, 7];
        let expected = vec![-1,1,4,3];
        let actual = Solution::min_bitwise_array(nums);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let nums = vec![11, 13, 31];
        let expected = vec![9, 12, 15];
        let actual = Solution::min_bitwise_array(nums);

        assert_eq!(expected, actual);
    }
}

fn get_min(n: i32) -> i32 {
    if n %2 == 0 {
        return -1;
    }
    let mut i = 0;
    while n & (1 << i) != 0 {
        i = i + 1;
    }

    return n ^ (1 << (i - 1));
}

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter().map(|it| get_min(it)).collect()
    }
}