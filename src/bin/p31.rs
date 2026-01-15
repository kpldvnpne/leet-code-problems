pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let mut nums = vec![1, 2, 3];
        let expected = vec![1, 3, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn test2() {
        let mut nums = vec![3, 2, 1];
        let expected = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn test3() {
        let mut nums = vec![1, 1, 5];
        let expected = vec![1, 5, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn test135() {
        let mut nums = vec![1, 3, 2];
        let expected = vec![2, 1, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn test166() {
        let mut nums = vec![2, 3, 1];
        let expected = vec![3, 1, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn test213() {
        let mut nums = vec![1, 5, 1];
        let expected = vec![5, 1, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(expected, nums);
    }
}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i > 0 && nums[i-1] >= nums[i] {
            i = i - 1;
        }
        if i == 0 {
            nums.sort();
            return;
        } else {
            i = i - 1;
            let last_index = nums.len() - 1;
            let mut other = last_index;
            while i < other && nums[other] <= nums[i] {
                other = other - 1;
            }
            let temp = nums[i];
            nums[i] = nums[other];
            nums[other] = temp;
            nums[i+1..(last_index+1)].sort();
        }
    }
}