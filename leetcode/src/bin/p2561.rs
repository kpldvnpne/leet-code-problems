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

  #[test]
  fn case_3() {
    assert_eq!(Solution::min_cost(vec![4, 4, 4, 4, 3], vec![5, 5, 5, 5, 3]), 8);
  }

  #[test]
  fn case_4() {
    Solution::min_cost(vec![84,80,43,8,80,88,43,14,100,88], vec![32,32,42,68,68,100,42,84,14,8]);
  }
}

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug)]
struct PositionedItem {
  item: i32,
  position: usize,
}

impl PartialEq for PositionedItem {
  fn eq(&self, other: &Self) -> bool {
    self.item == other.item && self.position == other.position
  }
}

impl PartialOrd for PositionedItem {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match self.item.partial_cmp(&other.item) {
      Some(core::cmp::Ordering::Equal) => {}
      ord => return ord,
    }
    self.position.partial_cmp(&other.position)
  }
}

impl Eq for PositionedItem {}
impl Ord for PositionedItem {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match self.item.cmp(&other.item) {
      core::cmp::Ordering::Equal => {
        self.position.cmp(&other.position)
      }
      ord => return ord
    }
  }
}

struct Link {
  from_upper: BTreeSet<PositionedItem>,
  from_lower: BTreeSet<PositionedItem>,
}

impl Default for Link {
  fn default() -> Self {
    return Link{
      from_upper: BTreeSet::new(),
      from_lower: BTreeSet::new(),
    };
  }
}

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
      let mut links: BTreeMap<i32, RefCell<Link>> = BTreeMap::new();
      for i in 0..(basket1.len()) {
        let a = basket1[i];
        let b = basket2[i];

        if !links.contains_key(&a) {
          links.insert(a, RefCell::new(Link::default()));
        }

        if !links.contains_key(&b) {
          links.insert(b, RefCell::new(Link::default()));
        }

        let mut link_a = links.get(&a).unwrap().borrow_mut();
        link_a.from_upper.insert(PositionedItem{
          item: b,
          position: i,
        });

        drop(link_a);

        let mut link_b = links.get(&b).unwrap().borrow_mut();
        link_b.from_lower.insert(PositionedItem {
          item: a,
          position: i,
        });
      }

      for (key, link) in links.iter() {
        let link = link.borrow();
        let total_len = link.from_upper.len() + link.from_lower.len();
        if total_len % 2 != 0 {
          println!("{key}, {total_len}");
          return -1; // Not possible
        }
      }

      let mut new_basket1 = basket1.clone();
      let mut new_basket2 = basket2.clone();

      for key in links.keys() {
        let mut link = links.get(key).unwrap().borrow_mut();
        let diff = link.from_upper.len() as i32 - link.from_lower.len() as i32;
        let to_change_count = (diff.abs() / 2) as usize;

        let source = if diff > 0 { link.from_upper.clone() } else { link.from_lower.clone() };

        for item in source.iter().rev().take(to_change_count) {
          if diff > 0 {
            link.from_upper.remove(item);
            link.from_lower.insert(*item);
          } else {
            link.from_lower.remove(item);
            link.from_upper.insert(*item);
          }

          let item_for_other = PositionedItem {
            item: *key,
            position: item.position
          };

          let mut other_linked = links.get(&item.item).unwrap().borrow_mut();
          if diff > 0 {
            other_linked.from_lower.remove(&item_for_other);
            other_linked.from_upper.insert(item_for_other);
          } else {
            other_linked.from_upper.remove(&item_for_other);
            other_linked.from_lower.insert(item_for_other);
          }

          // Swap in the basket
          let position = item.position;
          let temp = new_basket1[position];
          new_basket1[position] = new_basket2[position];
          new_basket2[position] = temp;
        }
      }

      // println!("{new_basket1:?}");
      // println!("{new_basket2:?}");

      let mut answer1: i64 = 0;

      for i in 0..new_basket1.len() {
        if new_basket1[i] != basket1[i] {
          // assert!(new_basket2[i] != new_basket2[i]);
          assert!(new_basket1[i] == basket2[i]);
          assert!(new_basket2[i] == basket1[i]);

          answer1 += std::cmp::min(new_basket1[i], new_basket2[i]) as i64;
        }
      }

      let temp = &new_basket1;
      let new_basket1 = &new_basket2;
      let new_basket2 = temp;

      let mut answer2: i64 = 0;
      for i in 0..new_basket1.len() {
        if new_basket1[i] != basket1[i] {
          // assert!(new_basket2[i] != new_basket2[i]);
          assert!(new_basket1[i] == basket2[i]);
          assert!(new_basket2[i] == basket1[i]);

          answer2 += std::cmp::min(new_basket1[i], new_basket2[i]) as i64;
        }
      }

      return std::cmp::min(answer1, answer2);
    }
}
