pub struct Solution;

fn main() {}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
      use std::collections::HashSet;
      let mut graph: Vec<HashSet<usize>> = Vec::with_capacity(n as usize);
      for _ in 0..n {
        graph.push(HashSet::new());
      }

      for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].insert(v);
        graph[v].insert(u);
      }

      use std::collections::VecDeque;
      let mut to_visit = VecDeque::new();
      to_visit.push_back(0);

      let mut visited = (0..n).map(|_| false).collect::<Vec<_>>();
      let mut valid = true;
      let mut count = 0;
      while let Some(node) = to_visit.pop_front() {
        if visited[node] {
          valid = false;
          break;
        }
        count += 1;
        visited[node] = true;

        let children = graph[node].iter().map(|&it| it).collect::<Vec<_>>();
        for child in children {
          graph[child].remove(&node);
          to_visit.push_back(child);
        }
      }

      if count != n {
        valid = false;
      }

      return valid;
    }
}
