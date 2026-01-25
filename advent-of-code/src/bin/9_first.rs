/*
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
*/

fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn rect_area_with(&self, other: &Self) -> i64 {
    (self.x - other.x + 1).abs() * (self.y - other.y + 1).abs()
  }
}

fn main() {
  let mut points = vec![];
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let [a, b] = line.split(',').map(|it| it.parse().unwrap()).collect::<Vec<_>>()[..2] else { panic!() };

    points.push(Point{x: a, y: b});
  }

  let mut max_area = 0;
  for i in 0..points.len()-1 {
    let a = &points[i];
    for j in i+1..points.len() {
      let b = &points[j];

      max_area = max_area.max(a.rect_area_with(b));
    }
  }

  println!("{max_area}");
}
