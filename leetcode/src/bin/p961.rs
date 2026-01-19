pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 3];
        let expected = 3;
        let actual = Solution::repeated_n_times(nums);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 1, 2, 5, 3, 2];
        let expected = 2;
        let actual = Solution::repeated_n_times(nums);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let nums = vec![5, 1, 5, 2, 5, 3, 5, 4];
        let expected = 5;
        let actual = Solution::repeated_n_times(nums);
        assert_eq!(expected, actual);
    }
}

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::with_capacity(nums.len() / 2 + 1);
        for num in nums {
            if set.contains(&num) {
                return num;
            } else {
                set.insert(num);
            }
        }

        panic!("The function should already return something");
    }
}