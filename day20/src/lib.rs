fn build_message(code: &str, key: i64) -> Vec<i64> {
    code.lines()
        .map(
            |l| l.parse::<i64>().unwrap() * key
        )
        .collect()
}

fn mix(mixing: &mut Vec<usize>, index: usize, mov: i64) {
    let cur_i = mixing.iter().position(|i| i == &index).unwrap();
    let extra = (mov % (mixing.len() - 1) as i64) as isize;
    let mut dest = cur_i as isize + extra;
    if dest < 0 {
        dest += (mixing.len() as isize) - 1;
    } else if dest >= mixing.len() as isize {
        dest -= (mixing.len() as isize) - 1;
    }
    let min = cur_i.min(dest as usize);
    let max = cur_i.max(dest as usize);
    let it: Box<dyn Iterator<Item = usize>> = if dest == max as isize {
        Box::new(min..max)
    } else {
        Box::new((min..max).rev())
    };
    for i in it {
        mixing.swap(i, i + 1);
    }
}

fn index_to_value(index: &Vec<usize>, value: &Vec<i64>) -> Vec<i64> {
    index.iter()
        .map(
            |i| value[*i]
        )
        .collect()
}

fn decrypt(code: &str, rounds: usize, key: i64) -> i64 {
    let message = build_message(&code, key);
    let mut mixing: Vec<usize> = (0..message.len()).collect();
    for _i in 0..rounds {
        message.iter().enumerate()
            .for_each(
                |(i, m)| {
                    mix(&mut mixing, i, *m);
                }
            );
    }
    let mixing: Vec<i64> = index_to_value(&mixing, &message);
    let mut it = mixing.into_iter().cycle();
    it.find(|i| i == &0);
    it.nth(999).unwrap() + it.nth(999).unwrap() + it.nth(999).unwrap()
}

pub fn decrypt_coordinates(input_path: &str, rounds: usize, key: i64) -> i64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => decrypt(&content, rounds, key),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

// pub fn run_idler_game_hungry_elephants(input_path: &str) -> u32 {
//     let content = std::fs::read_to_string(input_path);
//     match content {
//         Ok(content) => idler_game_hungry_elephants(&content),
//         Err(er) => {
//             println!("{}", er);
//             0
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP1: &str = 
r#"1
2
-3
3
-2
0
4"#;

    #[test]
    fn test_input1() {
        assert_eq!(decrypt(TEST_INP1, 1, 1), 3)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(decrypt(TEST_INP1, 10, 811589153), 1623178306)
    }
}