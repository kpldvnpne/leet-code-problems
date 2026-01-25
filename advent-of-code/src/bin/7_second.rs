fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

fn main() {
  let line = read_line();
  let line = line.trim();
  use std::collections::BTreeMap;
  let mut locations = BTreeMap::new();
  locations.entry(line.find('S').unwrap()).or_insert(1);

  // println!("{:?}", locations);

  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let mut new_locations = BTreeMap::new();
    for (location, freq) in locations {
      let ch_at_loc = line.chars().nth(location).unwrap();
      if ch_at_loc == '^' {

        if location > 0 {
          let entry = new_locations.entry(location - 1).or_default();
          *entry = *entry + freq;
        }

        if location < line.len() - 1 {
          let entry = new_locations.entry(location + 1).or_default();
          *entry = *entry + freq;
        }
      } else if ch_at_loc == '.' {
        let entry = new_locations.entry(location).or_default();
        *entry = *entry + freq;
      }
    }

    locations = new_locations;
    // println!("{}", locations.len());
  }

  println!("{}", locations.values().sum::<u64>());
}
