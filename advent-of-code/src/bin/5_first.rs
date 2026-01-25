fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

fn main() {
  let mut ranges = vec![];
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let [a, b] = line.split('-').into_iter().map(|it| it.parse::<u128>().unwrap()).collect::<Vec<_>>()[..2] else { panic!() };
    ranges.push((a, b));
  }

  let mut n_fresh = 0;
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let num = line.parse::<u128>().unwrap();
    let mut found = false;
    for range in &ranges {
      if range.0 <= num && num <= range.1 {
        found = true;
        break;
      }
    }

    if found {
      n_fresh += 1;
    }
  }

  println!("{}", n_fresh);
}
