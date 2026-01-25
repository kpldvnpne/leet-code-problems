fn read_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  return s;
}

fn main() {
  let mut table = vec![];
  loop {
    let line = read_line();
    let line = line.trim();

    if line.is_empty() {
      break;
    }

    let row: Vec<String> = line.split_whitespace().map(|it| it.to_string()).collect();
    table.push(row);
  }

  let mut total_result = 0;
  for j in 0..table[0].len() {
    let op = table[table.len() - 1][j].trim();
    let mut col_result: u64 = if op == "*" { 1 } else { 0 };
    for i in 0..(table.len()-1) {
      let number = table[i][j].trim().parse::<u64>().unwrap();
      col_result = if op == "*" {
        col_result * number
      } else {
        col_result + number
      };
    }
    total_result += col_result;
  }

  println!("{}", total_result);
}
