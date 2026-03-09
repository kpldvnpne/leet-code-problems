fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

#[derive(Clone)]
struct Point {
  x: i64,
  y: i64,
}

fn main() {
  let mut points = vec![];
  let mut x_min = i64::MAX;
  let mut x_max = i64::MIN;
  let mut y_min = i64::MAX;
  let mut y_max = i64::MIN;
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let [a, b] = line.split(',').map(|it| it.parse().unwrap()).collect::<Vec<_>>()[..2] else { panic!() };

    x_min = x_min.min(a);
    x_max = x_max.max(a);
    y_min = y_min.min(b);
    y_max = y_max.max(b);

    points.push(Point{x: a, y: b});
  }

  let mut hor_lines = vec![];
  let mut vert_lines = vec![];
  for i in 0..points.len() {
    let cur = &points[i];
    let next = &points[(i+1)%points.len()];
    if cur.x == next.x {
      let line = if cur.y < next.y { (cur, next) } else { (next, cur) };
      vert_lines.push(line);
    } if cur.y == next.y {
      let line = if cur.x < next.x { (cur, next) } else { (next, cur) };
      hor_lines.push(line);
    }
  }

  // Sweep vertically and save intersection in BTreeMap
  vert_lines.sort_by(|a, b| a.0.x.cmp(&b.0.x));
  let mut cur_intersection =
  // Sweep horizontally and save interection in BTreeMap
  // For every two point, find the interection both horizontally and vertically. If not exist in BTreeMap, then it is not eligible


}
