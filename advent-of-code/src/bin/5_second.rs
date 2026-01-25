fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

fn main() {
  let mut ranges: Vec<(u128, u128)> = vec![];
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let [a, b] = line.split('-').into_iter().map(|it| it.parse::<u128>().unwrap()).collect::<Vec<_>>()[..2] else { panic!() };

    assert!(a <= b);

    let mut new_ranges: Vec<(u128, u128)> = vec![];
    let mut inserted = false;

    for range in &ranges {
      let mut new_range = *range;

      let is_completely_before = b < new_range.0;
      if is_completely_before{
        if !inserted {
          new_ranges.push((a, b));
          inserted = true;
        }
        new_ranges.push(new_range);
        continue;
      }

      let is_completely_after = new_range.1 < a;
      if is_completely_after {
        new_ranges.push(new_range);
        continue;
      }

      // Has some overlap
      if !inserted {
        new_range.0 = new_range.0.min(a);
        new_range.1 = new_range.1.max(b);
        new_ranges.push(new_range);
        inserted = true;
      } else {
        // Edit the last range
        let last_index = new_ranges.len() - 1;
        let last_range = &mut new_ranges[last_index];
        last_range.1 = last_range.1.max(new_range.1);
      }

    }

    if !inserted {
      new_ranges.push((a, b));
    }

    ranges = new_ranges;
  }

  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }
  }

  let mut ans = 0;
  for range in ranges {
    // println!("{:?}", range);
    ans += range.1 - range.0 + 1;
  }

  println!("{}", ans);
}
