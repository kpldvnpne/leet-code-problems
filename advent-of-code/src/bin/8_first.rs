fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

#[derive(Debug)]
struct Point(i64, i64, i64);

impl Point {
  fn dist_with(&self, other: &Point) -> f64 {
    f64::sqrt(((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
  }
}

fn main() {
  let top_n = read_line();
  let top_n: usize = top_n.trim().parse().unwrap();

  let mut points = vec![];
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let [a, b, c] = line.split(',').map(|it| it.parse::<i64>().unwrap()).collect::<Vec<_>>()[..3] else { panic!() };
    points.push(Point(a, b, c));
  }

  let mut distances = vec![];
  for i in 0..points.len()-1 {
    let a = &points[i];
    for j in i+1..points.len() {
      let b = &points[j];

      distances.push((a.dist_with(&b), i, j));
    }
  }

  distances.sort_by(|a, b| a.0.total_cmp(&b.0));
  use std::collections::HashSet;
  let mut groups: Vec<HashSet<usize>>  = (0..points.len()).into_iter().map(|it| {
    let mut set = HashSet::new();
    set.insert(it);
    set
  }).collect();

  // let mut n_connections = 0;
  for distance in &distances[..top_n] {
    let (_, i, j) = distance;
    // println!("{} {} {:?} {:?}", i, j, points[*i], points[*j]);
    let mut new_groups: Vec<HashSet<usize>> = vec![];
    let mut groups_to_merge = vec![];
    for group in groups {
      if group.contains(&i) || group.contains(&j) {
        groups_to_merge.push(group);
      } else {
        new_groups.push(group);
      }
    }

    let new_group = match groups_to_merge.len() {
      1 => groups_to_merge[0].clone(),
      2 => groups_to_merge[0].union(&groups_to_merge[1]).map(|it| *it).collect(),
      _ => unreachable!(),
    };

    // if groups_to_merge.len() == 2 {
    //   n_connections += 1;
    //   println!("Connected");
    // }

    new_groups.push(new_group);

    groups = new_groups;

    // println!("{:?}", groups);

    // if n_connections == top_n {
    //   break;
    // }
  }

  // println!("{:?}", groups);

  let mut group_sizes: Vec<_> = groups.into_iter().map(|g| g.len() as u128).collect();
  // println!("{:?}", group_sizes);
  group_sizes.sort_unstable_by(|a, b| b.cmp(&a));
  let ans: u128 = group_sizes[..3].iter().product();
  println!("{}", ans);
}
