fn find_marker(transmission: &str, window_size: usize) -> usize {
    let mut rolling_window = vec!['\0'; window_size];
    for (i, c) in transmission.chars().enumerate() {
        rolling_window.pop();
        rolling_window.insert(0, c);
        let set = std::collections::BTreeSet::from_iter(rolling_window.iter());
        if set.len() == rolling_window.len() && i >= window_size{
            return i + 1;
        }
    }
    0
}

pub fn decode_packet(input_path: &str) -> usize {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => find_marker(&content, 4),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

pub fn decode_message(input_path: &str) -> usize {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => find_marker(&content, 14),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_INP2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_INP3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_INP4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_INP5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_input1() {
        assert_eq!(find_marker(TEST_INP1, 4), 7)
    }

    #[test]
    fn test_input2() {
        assert_eq!(find_marker(TEST_INP2, 4), 5)
    }

    #[test]
    fn test_input3() {
        assert_eq!(find_marker(TEST_INP3, 4), 6)
    }

    #[test]
    fn test_input4() {
        assert_eq!(find_marker(TEST_INP4, 4), 10)
    }

    #[test]
    fn test_input5() {
        assert_eq!(find_marker(TEST_INP5, 4), 11)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(find_marker(TEST_INP1, 14), 19)
    }

    #[test]
    fn test_input2_part2() {
        assert_eq!(find_marker(TEST_INP2, 14), 23)
    }

    #[test]
    fn test_input3_part2() {
        assert_eq!(find_marker(TEST_INP3, 14), 23)
    }

    #[test]
    fn test_input4_part2() {
        assert_eq!(find_marker(TEST_INP4, 14), 29)
    }

    #[test]
    fn test_input5_part2() {
        assert_eq!(find_marker(TEST_INP5, 14), 26)
    }
}