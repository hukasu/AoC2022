use std::ops::{RangeInclusive, Not};

fn assignment_to_range(assignment: &str) -> RangeInclusive<usize> {
    assignment.split_once("-")
        .and_then(|(a, b)| Some((a.trim(), b.trim())))
        .and_then(|(a, b)| Some(a.parse().unwrap_or(std::usize::MAX)..=b.parse().unwrap_or(std::usize::MAX)))
        .unwrap()
}

fn pair_to_assignments(pair: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    pair.split_once(",")
        .map(|(a, b)| (assignment_to_range(a), assignment_to_range(b)))
        .unwrap()
}

fn assignments_superset(assign1: RangeInclusive<usize>, assign2: RangeInclusive<usize>) -> bool {
    let assign1 = std::collections::BTreeSet::from_iter(assign1);
    let assign2 = std::collections::BTreeSet::from_iter(assign2);
    assign1.is_superset(&assign2) || assign1.is_subset(&assign2)
}

fn find_assignments_supersets(pairs: &str) -> u32 {
    pairs.split_terminator("\n")
        .map(pair_to_assignments)
        .map(|(a, b)| assignments_superset(a, b) as u32)
        .sum()
}

pub fn organize_assignments_supersets(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => find_assignments_supersets(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn assignments_overlap(assign1: RangeInclusive<usize>, assign2: RangeInclusive<usize>) -> bool {
    let assign1 = std::collections::BTreeSet::from_iter(assign1);
    let assign2 = std::collections::BTreeSet::from_iter(assign2);
    assign1.is_disjoint(&assign2).not()
}

fn find_assignments_overlaps(pairs: &str) -> u32 {
    pairs.split_terminator("\n")
        .map(pair_to_assignments)
        .map(|(a, b)| assignments_overlap(a, b) as u32)
        .sum()
}

pub fn organize_assignments_overlaps(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => find_assignments_overlaps(&content),
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
r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_input() {
        assert_eq!(find_assignments_supersets(TEST_INP), 2)
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(find_assignments_overlaps(TEST_INP), 4)
    }
}