fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

fn main() {
  let line = read_line();
  let line = line.trim();
  use std::collections::BTreeSet;
  let mut locations = BTreeSet::new();
  locations.insert(line.find('S').unwrap());

  // println!("{:?}", locations);

  let mut n_splits: u32 = 0;
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let mut new_locations = BTreeSet::new();
    for location in locations {
      let ch_at_loc = line.chars().nth(location).unwrap();
      if ch_at_loc == '^' {
        n_splits += 1;

        if location > 0 {
          new_locations.insert(location - 1);
        }

        if location < line.len() - 1 {
          new_locations.insert(location + 1);
        }
      } else if ch_at_loc == '.' {
        new_locations.insert(location);
      }
    }

    locations = new_locations;
    // println!("{:?}", locations);
  }

  println!("{}", n_splits);
}
