

fn read_line() -> String {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    return str;
}

fn read_int_line() -> i32 {
    read_line().trim().parse().unwrap()
}

fn read_int_array_line() -> Vec<i32> {
    read_line().split_ascii_whitespace().map(|it| it.trim().parse().unwrap()).collect()
}

fn main() {
    let t: i32 = read_int_line();
    for _ in 0..t {
        let n = read_int_line() as usize;
        let a = read_int_array_line();
        let b = read_int_array_line();
        let c = read_int_array_line();

        let mut b_c_changes = 0;
        for diff in 0..n {
            let mut possible = true;
            for i in 0..n {
                if !(b[i] < c[(i + diff) % n]) {
                    possible = false;
                    break;
                }
            }

            if possible {
                b_c_changes += 1
            }
        }

        let mut a_b_changes = 0;
        for diff in 0.. n {
            let mut possible = true;
            for i in 0..n {
                if !(a[i] < b[(i + diff) % n]) {
                    possible = false;
                    break;
                }
            }

            if possible {
                a_b_changes += 1
            }
        }

        let total_changes = n * a_b_changes * b_c_changes;

        println!("{total_changes}");
    }
}