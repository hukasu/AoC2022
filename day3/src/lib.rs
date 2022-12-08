fn char_priority(c: char) -> u32 {
    match c.cmp(&'a') {
        std::cmp::Ordering::Less => (c as u32) - ('A' as u32) + 27,
        _ => (c as u32) - ('a' as u32) + 1
    }
}

fn calculate_rucksack_error(rucksack: &str) -> u32 {
    let (comp1, comp2) = rucksack.trim().split_at(rucksack.len() / 2);
    let comp1_set = std::collections::BTreeSet::from_iter(comp1.chars());
    let comp2_set = std::collections::BTreeSet::from_iter(comp2.chars());
    let intersect = comp1_set.intersection(&comp2_set);
    intersect.into_iter()
        .map(|c| char_priority(*c))
        .sum()
}

fn calculate_rucksacks_errors(rucksacks: &str) -> u32 {
    let rucksacks = rucksacks.split_terminator("\n");
    rucksacks
        .map(
            |rucksack| calculate_rucksack_error(rucksack)
        )
        .sum()
}

pub fn organize_rucksacks(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => calculate_rucksacks_errors(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn calculate_badge(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> u32 {
    let ruck1 = std::collections::BTreeSet::from_iter(rucksack1.trim().chars());
    let ruck2 = std::collections::BTreeSet::from_iter(rucksack2.trim().chars());
    let ruck3 = std::collections::BTreeSet::from_iter(rucksack3.trim().chars());
    std::collections::BTreeSet::from_iter(
        ruck1.intersection(&ruck2).map(|c| *c)
    )
        .intersection(&ruck3)
        .map(|c| char_priority(*c))
        .sum()
}

fn calculate_badges(rucksacks: &str) -> u32 {
    let rucksacks: Vec<&str> = rucksacks.split_terminator("\n").collect();
    rucksacks
        .chunks(3)
        .map(
            |group| {
                if let [a, b, c] = group {
                    calculate_badge(a, b, c)
                } else {
                    panic!("Chunk with more/less than 3 elfs.")
                }
            }
        )
        .sum()
}

pub fn organize_badges(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => calculate_badges(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP: &str =
r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_input() {
        assert_eq!(calculate_rucksacks_errors(TEST_INP), 157)
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(calculate_badges(TEST_INP), 70)
    }
}