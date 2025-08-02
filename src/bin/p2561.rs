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

use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;
use std::hash::Hash;

struct Link {
  upper: HashMultiSet<i32>,
  lower: HashMultiSet<i32>,
}

impl Default for Link {
  fn default() -> Self {
    return Link{
      upper: HashMultiSet::new(),
      lower: HashMultiSet::new(),
    };
  }
}

#[derive(Debug)]
struct HashMultiSet<K> {
  map: HashMap<K, i32>
}

impl<K> HashMultiSet<K>
where K: Eq + Hash {
    fn insert(self: &mut Self, item: K) {
      let count = self.map.get(&item).unwrap_or(&0);
      let new_count = count + 1;
      self.map.insert(item, new_count);
    }

    fn remove(self: &mut Self, item: K) {
      let count = self.map.get(&item).unwrap();
      let new_count = count - 1;
      if new_count == 0 {
        self.map.remove(&item);
      } else {
        self.map.insert(item, new_count);
      }
    }

    fn len(self: Self) -> usize {
      self.map.len()
    }

    fn new() -> Self {
      return Self {
        map: HashMap::new()
      };
    }

    fn iter(self: Self) -> Vec<K> {
      return self.map.keys().collect().clone();
    }
}

// fn sort_descending<T: Ord>(deque: VecDeque<T>) -> VecDeque<T> {
//     let mut vec: Vec<T> = deque.into_iter().collect();
//     vec.sort_by(|a, b| b.cmp(a)); // Descending order
//     vec.into_iter().collect()
// }

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
        link.upper.insert(b);

        let link: &mut Link = links.get_mut(&b).unwrap();
        link.lower.insert(a);
      }

      let mut unbalanced_items: HashSet<i32> = HashSet::new();
      for (key, link) in links.iter() {
        println!("{key}: {:?}", &link.upper);
        println!("{key}: {:?}", &link.lower);
        let total_len = link.upper.len() + link.lower.len();
        if total_len % 2 != 0 {
          println!("{key}, {total_len}");
          return -1; // Not possible
        }
        if link.upper.len() as i32 - link.lower.len() as i32 != 0 {
          unbalanced_items.insert(*key);
        }
      }

      let mut pruned_links: HashMap<i32, RefCell<Link>> = HashMap::new();
      for item in unbalanced_items.iter() {
        let link = links.get(item).unwrap();
        let new_upper = link.upper.iter().filter(|x| unbalanced_items.contains(*x) && *x != item).map(|x| *x).collect();
        let new_lower =  link.lower.iter().filter(|x| unbalanced_items.contains(*x) && *x != item).map(|x| *x).collect();
        pruned_links.insert(*item, RefCell::new(Link{
          upper: new_upper,
          lower: new_lower,
        }));
      }

      let mut keys_sorted: Vec<i32> = pruned_links.keys().cloned().collect::<Vec<i32>>();
      keys_sorted.sort();
      let mut to_flip_list: Vec<(i32, i32)> = vec![];
      for key in keys_sorted {
        let link = pruned_links.get(&key).unwrap().borrow();
        link.upper.iter().for_each(|x| {
          to_flip_list.push((key, *x));
          pruned_links.get(&x).unwrap().borrow_mut().lower.remove(&key);
        });

        link.lower.iter().for_each(|x| {
          to_flip_list.push((*x, key));
          pruned_links.get(&x).unwrap().borrow_mut().upper.remove(&key);
        });
      }

      to_flip_list.sort_by(|a, b| std::cmp::min(a.0, a.1).cmp(&std::cmp::min(b.0, b.1)));

      return to_flip_list.iter().take(to_flip_list.len() / 2).map(|a| std::cmp::min(a.0, a.1) as i64).sum();
    }
}
