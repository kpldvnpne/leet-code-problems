struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let apple = vec![1, 3, 2];
        let capacity = vec![4, 3, 1, 5, 2];

        assert_eq!(2, Solution::minimum_boxes(apple, capacity));
    }

    #[test]
    fn test2() {
        let apple = vec![5, 5, 5];
        let capacity = vec![2, 4, 2, 7];

        assert_eq!(4, Solution::minimum_boxes(apple, capacity));
    }
}

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let total_apples = apple.iter().sum();
        capacity.sort_by(|a, b| b.cmp(a));

        let mut cum_capacity = 0;
        let mut i = 0;
        while i < capacity.len() && cum_capacity < total_apples {
            cum_capacity += capacity[i];
            i += 1;
        }

        return i as i32;
    }
}