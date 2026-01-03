struct Solution;

fn main() {
    println!("Len = {}", get_first_row_combinations().len());
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let n = 1;
        let expected = 12;
        let actual = Solution::num_of_ways(n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let n = 5000;
        let expected = 30228214;
        let actual = Solution::num_of_ways(n);
        assert_eq!(expected, actual);
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum Color {
    Red,
    Yellow, 
    Green,
}

type Triplet = (Color, Color, Color);

fn get_first_row_combinations() -> Vec<Triplet> {
    let colors = [Color::Red, Color::Yellow, Color::Green];
    let mut output = vec![];
    for color1 in &colors {
        for color2 in &colors {
            if color2 == color1 { continue; }
            for color3 in &colors {
                if color3 == color2 { continue; }
                output.push((*color1, *color2, *color3));
            }
        }
    }
    return output;
}

fn get_conditional_combinations(previous_row: &Triplet, possible_combinations: &Vec<Triplet>) -> Vec<Triplet> {
    let mut allowed = vec![];
    for candidate in possible_combinations.iter() {
        if previous_row.0 != candidate.0 && previous_row.1 != candidate.1 && previous_row.2 != candidate.2 {
            allowed.push(*candidate)
        }
    }
    return allowed;
}

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let first_row_combinations = get_first_row_combinations();
        use std::collections::HashMap;
        let mut allowed_map: HashMap<Triplet, Vec<Triplet>> = HashMap::new();
        for key in &first_row_combinations {
            let val = get_conditional_combinations(key, &first_row_combinations);
            allowed_map.insert(*key, val);
        }

        let mut map = HashMap::new();
        for item in &first_row_combinations {
            map.insert(*item, 1);
        }

        let mut prev_map: HashMap<Triplet, i64> = map;
        for _ in 0..(n-1) {
            let mut cur_map = HashMap::new();
            for (item, value) in prev_map.iter() {
                let allowed_triplets = allowed_map.get(item).unwrap();
                for allowed_triplet in allowed_triplets {
                    let entry = cur_map.entry(*allowed_triplet).or_insert(0);
                    *entry = (*entry + value) % (1_000_000_000 + 7);
                }
            }
            prev_map = cur_map;
        }

        let mut sum = 0;
        for val in prev_map.values() {
            sum = (sum + val) % (1_000_000_000 + 7);
        }
        
        return sum as i32;
    }
}