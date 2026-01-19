pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::Solution;

    #[test]
    fn test1() {
        let events = vec![
            vec![1,3,2],
            vec![4,5,2],
            vec![2,4,3]
        ];

        let output = Solution::max_two_events(events);
        assert_eq!(output, 4);
    }

    #[test]
    fn test2() {
        let events = vec![
            vec![1, 3, 2],
            vec![4, 5, 2],
            vec![1, 5, 5],
        ];

        let output = Solution::max_two_events(events);
        assert_eq!(output, 5);
    }

    #[test]
    fn test3() {
        let events = vec![
            vec![1, 5, 3],
            vec![1, 5, 1],
            vec![6, 6, 5],
        ];

        let output = Solution::max_two_events(events);
        assert_eq!(output, 8);
    }
}

trait BinarySearchAfter {
    fn after(&self, start_time: i32, left: usize, right: usize) -> usize;
}

impl BinarySearchAfter for Vec<Vec<i32>> {
    fn after(&self, time: i32, mut left: usize, mut right: usize) -> usize {
        while left != right {
            let mid: usize = (left + right) / 2;
            let mid_start_time = self[mid][0];
            if time < mid_start_time {
                right = mid; 
                
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
}
impl Solution {
    pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut max_values: Vec<Vec<i32>> = vec![vec![]; events.len()];
        let mut max_value = i32::MIN;
        for (i, event) in events.iter().enumerate().rev() {
            if event[2] > max_value {
                max_value = event[2];
            }
            max_values[i].append(&mut vec![event[0], max_value])
        }

        let mut max_total_values = i32::MIN;
        for (i, event) in events.iter().enumerate() {
            let cur_value = event[2];
            let cur_end_time = event[1];

            // Find max value after cur_end_time
            let max_value_index = max_values.after(cur_end_time, i, max_values.len());
            let max_value = if max_value_index < max_values.len() {max_values[max_value_index][1]} else {0};

            let total_values = cur_value + max_value;
            max_total_values = std::cmp::max(total_values, max_total_values);
        }

        return max_total_values;

    }
}