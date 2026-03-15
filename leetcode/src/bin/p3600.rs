pub struct Solution;

fn main() {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let n = 3;
    let edges = vec![
      vec![0, 1, 2, 1],
      vec![1, 2, 3, 0],
    ];
    let k = 1;

    let expected = 2;
    let actual = Solution::max_stability(n, edges, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test2() {
    let n = 3;
    let edges = vec![
      vec![0, 1, 4, 0],
      vec![1, 2, 3, 0],
      vec![0, 2, 1, 0],
    ];
    let k = 2;

    let expected = 6;
    let actual = Solution::max_stability(n, edges, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test3() {
    let n = 3;
    let edges = vec![
      vec![0, 1, 1, 1],
      vec![1, 2, 1, 1],
      vec![2, 0, 1, 1],
    ];
    let k = 0;

    let expected = -1;
    let actual = Solution::max_stability(n, edges, k);

    assert_eq!(expected, actual);
  }



  #[test]
  fn test4() {
    let n = 4;
    let edges = vec![
      vec![0,1,23967,1],
      vec![1,2,85332,0],
      vec![0,3,71167,1],
      vec![2,3,8451,0],
      vec![0,2,94942,0],
      vec![1,3,138,1],
    ];
    let k = 3;

    let expected = -1;
    let actual = Solution::max_stability(n, edges, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test5() {
    let n = 2;
    let edges = vec![
      vec![0,1,87487,0],
    ];
    let k = 0;

    let expected = 87487;
    let actual = Solution::max_stability(n, edges, k);

    assert_eq!(expected, actual);
  }

  #[test]
  fn test6() {
    let n = 3;
    let edges = vec![
      vec![0,1,55839,0],
      vec![0,2,39867,0],
      vec![1,2,62840,0]
    ];
    let k = 1;

    let expected = 62840;
    let actual = Solution::max_stability(n, edges, k);

    assert_eq!(expected, actual);
  }

}

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
      fn merge_set(dest: usize, src: usize, sets: &mut Vec<(i32, Vec<i32>)>) {
        let src_vec = std::mem::take(&mut sets[src].1);
        for item in src_vec {
          sets[dest].1.push(item);
          sets[item as usize].0 = sets[dest].0;
        }
      }

      let mut n_sets = n;
      let mut sets = (0..n).map(|it| { (it, vec![it])}).collect::<Vec<_>>();

      let mut min_strength = i32::MAX;
      let must_edges = edges.iter().filter(|it| it[3] == 1);
      for edge in must_edges {
        // let to_print = sets.iter().map(|it| it.0).collect::<Vec<_>>();
        // println!("Set assigned: {:?}", to_print);
        // println!("Edge: {:?}", edge);

        let [u, v, s] = edge[..3] else { panic!() };
        let (u, v) = (u as usize, v as usize);
        if sets[u].0 == sets[v].0 {
          return -1;
        }

        let set_u = sets[u].0 as usize;
        let set_v = sets[v].0 as usize;
        let size_u = sets[set_u].1.len();
        let size_v = sets[set_v].1.len();

        if size_u > size_v {
          merge_set(set_u, set_v, &mut sets);
        } else {
          merge_set(set_v, set_u, &mut sets);
        }
        n_sets -= 1;
        min_strength = min_strength.min(s);

        // let to_print = sets.iter().map(|it| it.0).collect::<Vec<_>>();
        // println!("Set assigned: {:?}", to_print);
      }

      let mut non_must_edges = edges.iter().filter(|it| it[3] == 0).collect::<Vec<_>>();
      non_must_edges.sort_by(|a, b| b[2].cmp(&a[2]));
      let mut iter = non_must_edges.iter();
      let mut strengths = vec![];
      while n_sets > 1 && let Some(edge) = iter.next() {
        let [u, v, s] = edge[..3] else { panic!() };

        let (u, v) = (u as usize, v as usize);
        if sets[u].0 == sets[v].0 {
          // println!("Skipping edges: {:?}", edge);
          continue;
        }

        let set_u = sets[u].0 as usize;
        let set_v = sets[v].0 as usize;
        let size_u = sets[set_u].1.len();
        let size_v = sets[set_v].1.len();

        if size_u > size_v {
          merge_set(set_u, set_v, &mut sets);
        } else {
          merge_set(set_v, set_u, &mut sets);
        }
        n_sets -= 1;
        strengths.push(s);
      }

      if n_sets > 1 {
        return -1;
      }

      // println!("{:?}", strengths);

      let mut k = k;
      for &s in strengths.iter().rev() {
        if k > 0 {
          min_strength = min_strength.min(s * 2);
          k -= 1;
        } else {
          min_strength = min_strength.min(s);
        }
      }

      return min_strength;

    }
}
