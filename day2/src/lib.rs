enum Play {
    Rock,
    Paper,
    Scissor
}

fn enemy_play(p: &str) -> Play {
    match p.trim() {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissor,
        _ => panic!("Unknown play")
    }
}

fn player_play(p: &str) -> Play {
    match p.trim() {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissor,
        _ => panic!("Unknown play")
    }
}

fn player_play_updated(e: &Play, p: &str) -> Play {
    match (e, p.trim()) {
        (Play::Rock, "X") => Play::Scissor,
        (Play::Rock, "Y") => Play::Rock,
        (Play::Rock, "Z") => Play::Paper,
        (Play::Paper, "X") => Play::Rock,
        (Play::Paper, "Y") => Play::Paper,
        (Play::Paper, "Z") => Play::Scissor,
        (Play::Scissor, "X") => Play::Paper,
        (Play::Scissor, "Y") => Play::Scissor,
        (Play::Scissor, "Z") => Play::Rock,
        _ => panic!("Unknown play")
    }
}

fn round_score(e: &Play, p: &Play) -> u32 {
    match (e, p) {
        (Play::Rock, Play::Rock) => 1 + 3,
        (Play::Rock, Play::Paper) => 2 + 6,
        (Play::Rock, Play::Scissor) => 3 + 0,
        (Play::Paper, Play::Rock) => 1 + 0,
        (Play::Paper, Play::Paper) => 2 + 3,
        (Play::Paper, Play::Scissor) => 3 + 6,
        (Play::Scissor, Play::Rock) => 1 + 6,
        (Play::Scissor, Play::Paper) => 2 + 0,
        (Play::Scissor, Play::Scissor) => 3 + 3,
    }
}

fn compute_round(input: &str) -> u32 {
    let mut c = input.split_whitespace();
    round_score(&enemy_play(c.next().unwrap()), &player_play(c.next().unwrap()))
}

fn compute_round_updated(input: &str) -> u32 {
    let mut c = input.split_whitespace();
    let e = enemy_play(c.next().unwrap());
    round_score(&e, &player_play_updated(&e, c.next().unwrap()))
}

pub fn calculate_strategy_guide_score(input_path: &str) -> Option<u32> {
    if let Ok(input) = std::fs::read_to_string(input_path) {
        let score: u32 = input.split_terminator("\n")
            .into_iter()
            .map(compute_round)
            .sum();
        Some(score)
    } else {
        None
    }
}

pub fn calculate_strategy_guide_score_updated(input_path: &str) -> Option<u32> {
    if let Ok(input) = std::fs::read_to_string(input_path) {
        let score: u32 = input.split_terminator("\n")
            .into_iter()
            .map(compute_round_updated)
            .sum();
        Some(score)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "A Y";
        assert_eq!(compute_round(input), 8)
    }

    #[test]
    fn test_2() {
        let input = "B X";
        assert_eq!(compute_round(input), 1)
    }

    #[test]
    fn test_3() {
        let input = "C Z";
        assert_eq!(compute_round(input), 6)
    }

    #[test]
    fn test_part2_1() {
        let input = "A Y";
        assert_eq!(compute_round_updated(input), 4)
    }

    #[test]
    fn test_part2_2() {
        let input = "B X";
        assert_eq!(compute_round_updated(input), 1)
    }

    #[test]
    fn test_part2_3() {
        let input = "C Z";
        assert_eq!(compute_round_updated(input), 7)
    }
}