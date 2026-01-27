pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  trait ToVecOfVec<T> {
    fn to_vec_of_vec(self) -> Vec<Vec<T>>;
  }

  impl<I, J, T> ToVecOfVec<T> for I
      where I: IntoIterator<Item=J>, J: IntoIterator<Item = T>
  {
    fn to_vec_of_vec(self) -> Vec<Vec<T>> {
    self.into_iter()
      .map(|inner| inner.into_iter().collect())
      .collect()
    }
  }

  #[test]
  fn test1() {
    let n = 4;
    let edges = [[0,1,3],[3,1,1],[2,3,4],[0,2,2]].to_vec_of_vec();

    let expected = 5;
    let actual = Solution::min_cost(n, edges);

    assert_eq!(expected, actual);
  }

    #[test]
  fn test2() {
    let n = 4;
    let edges = [[0,2,1],[2,1,1],[1,3,1],[2,3,3]].to_vec_of_vec();

    let expected = 3;
    let actual = Solution::min_cost(n, edges);

    assert_eq!(expected, actual);
  }
}

#[derive(Eq, PartialEq)]
struct Distance {
  distance: u32,
  from: usize,
  to: usize,
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      other.distance.cmp(&self.distance) // Reverse order so that Binary Heap is a min heap
    }
}

use std::collections::{BTreeMap, BinaryHeap};
impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
      let n = n as usize;
      let mut graph = vec![BTreeMap::new(); n as usize];

      // Construct the graph from the edges
      for edge in edges {
        let [u, v, w] = edge[..3] else { panic!() };
        let (u, v, w) = (u as usize, v as usize, w as u32);
        let entry = graph[u].entry(v).or_insert(w);
        *entry = std::cmp::min(*entry, w);
        let entry = graph[v].entry(u).or_insert(2 * w);
        *entry = std::cmp::min(*entry, 2 * w);
      }

      // Get shortest path from 0 to n-1 using Djikstra's Algorithm
      let mut visited = vec![false; n];
      visited[0] = true;
      let mut queue = BinaryHeap::new();
      for (to, weight) in graph[0].iter() {
        queue.push(Distance{
          distance: *weight,
          from: 0,
          to: *to,
        });
      }

      let mut ans = 0;
      while !queue.is_empty() {
        let shortest = queue.pop().unwrap();
        let vertex = shortest.to;
        if !visited[vertex] {
          visited[vertex] = true;

          if vertex == n-1 {
            ans = shortest.distance;
            break;
          }

          for (to, weight) in graph[vertex].iter() {
            if visited[*to] {
              continue;
            }
            queue.push(Distance {
              distance: shortest.distance + *weight,
              from: vertex,
              to: *to,
            });
          }
        }
      }

      if visited[n-1] {
        return ans as i32;
      } else {
        return -1;
      }
    }
}
