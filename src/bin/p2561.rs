pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_1() {
    assert_eq!(Solution::min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]), 1);
  }

  #[test]
  fn case_2() {
    assert_eq!(Solution::min_cost(vec![2, 3, 4, 1], vec![3, 2, 5, 1]), -1);
  }
}

use std::collections::HashMap;
use std::collections::VecDeque;

struct Link {
  upper: VecDeque<i32>,
  lower: VecDeque<i32>,
}

impl Default for Link {
  fn default() -> Self {
    return Link{
      upper: VecDeque::new(),
      lower: VecDeque::new(),
    };
  }
}



impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
      let mut links: HashMap<i32, Link> = HashMap::new();
      for i in 0..(basket1.len()) {
        let a = basket1[i];
        let b = basket2[i];

        if !links.contains_key(&a) {
          links.insert(a, Link::default());
        }

        if !links.contains_key(&b) {
          links.insert(b, Link::default());
        }

        let link: &mut Link = links.get_mut(&a).unwrap();
        link.upper.push_back(b);

        let link: &mut Link = links.get_mut(&b).unwrap();
        link.lower.push_back(a);
      }

      for link in links.values() {
        let total_len = link.upper.len() + link.lower.len();
        if total_len % 2 != 0 {
          return -1; // Not possible
        }
      }

      for link in links.values_mut() {
        link.upper.make_contiguous().reverse();
        link.lower.make_contiguous().reverse();
      }

      let mut keys_sorted: Vec<i32> = links.keys().cloned().collect::<Vec<i32>>();
      keys_sorted.sort();

      let mut total_cost: i64 = 0;

      for key in keys_sorted {
        let link = links.get(&key).unwrap();
        let diff = link.upper.len() as i32 - link.lower.len() as i32;
        if diff == 0 {
          continue;
        }
        let list_to_look = if diff > 0 { &link.upper.clone() } else { &link.lower.clone() };
        let swaps_needed: i32 = i32::abs(diff) / 2;

        total_cost += (swaps_needed * key) as i64;

        for i in 0..(swaps_needed as usize) {
          let other_number = list_to_look[i];
          let other_link = links.get_mut(&other_number).unwrap();
          if diff > 0 {
            let _: i32 = other_link.lower.pop_back().unwrap();
            // assert_eq!(popped, other_number);
          } else {
            let _ = other_link.upper.pop_back().unwrap();
            // assert_eq!(popped, other_number);
          }
        }
      }

      return total_cost;
    }
}
