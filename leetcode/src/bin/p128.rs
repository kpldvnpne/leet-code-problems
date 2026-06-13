pub struct Solution;

fn main() {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
          return 0;
        }

        if nums.len() == 1 {
          return 1;
        }

        struct Container {
          left: Option<i32>,
          right: Option<i32>,
          visited: bool
        }

        use std::collections::HashMap;
        let mut hashmap: HashMap<i32, Container> = HashMap::with_capacity(nums.len());
        for &num in &nums {
          let mut new_entry = Container {
            left: None,
            right: None,
            visited: false,
          };

          if let Some(left) = hashmap.get_mut(&(num - 1)) {
            left.right = Some(num);
            new_entry.left = Some(num - 1);
          }

          if let Some(right) = hashmap.get_mut(&(num + 1)) {
            right.left = Some(num);
            new_entry.right = Some(num + 1);
          }

          hashmap.insert(num, new_entry);
        }

        let mut max_consecutive = 1;
        for &num in &nums {
          let container = hashmap.get_mut(&num).unwrap();
          if !container.visited {
            container.visited = true;
            let mut consecutive = 1;
            let mut left = container.left;
            let mut right = container.right;

            while let Some(some_left) = left {
              consecutive += 1;
              let left_container = hashmap.get_mut(&some_left).unwrap();
              left_container.visited = true;
              left = left_container.left;
            }

            while let Some(some_right) = right {
              consecutive += 1;
              let right_container = hashmap.get_mut(&some_right).unwrap();
              right_container.visited = true;
              right = right_container.right;
            }

            max_consecutive = max_consecutive.max(consecutive);
          }
        }

        return max_consecutive;
    }
}
