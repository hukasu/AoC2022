use std::{collections::{HashMap}};

#[derive(Debug)]
enum Fill {
    Rock,
    Sand
}

fn build_cave(cave: &str) -> HashMap<(usize, usize), Fill> {
    cave.lines()
        .flat_map(
            |l| {
                let mut coords = l.split_terminator(" -> ")
                    .map(
                        |coord| {
                            let (x, y) = coord.split_once(",").unwrap();
                            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                        }
                    );
                let first = coords.next().unwrap();
                coords.fold(
                        (vec![], first),
                        |(mut v, prev), cur| {
                            match (prev.0.cmp(&cur.0), prev.1.cmp(&cur.1)) {
                                (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                                    (prev.1..=cur.1).for_each(
                                        |i| v.push((prev.0, i))
                                    )
                                },
                                (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                                    (cur.1..=prev.1).for_each(
                                        |i| v.push((prev.0, i))
                                    )
                                },
                                (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal) => {
                                    v.push((prev.0, prev.1))
                                },
                                (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                                    (prev.0..=cur.0).for_each(
                                        |i| v.push((i, prev.1))
                                    )
                                },
                                (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                                    (cur.0..=prev.0).for_each(
                                        |i| v.push((i, prev.1))
                                    )
                                },
                                _ => panic!("Diagonal rock wall '{:?}' -> '{:?}'", prev, cur)
                            }
                            (v, cur)
                        }
                    )
                    .0
            }
        )
        .fold(
            HashMap::new(),
            |mut hm, cur| {
                hm.insert(cur, Fill::Rock);
                hm
            }
        )
}

fn abyss_height(cave: &HashMap<(usize, usize), Fill>) -> usize {
    cave.iter().max_by(|(a, _), (b, _)| a.1.cmp(&b.1)).unwrap().0.1
}

#[allow(dead_code)]
fn print_cave(cave: &HashMap<(usize, usize), Fill>, include_floor: bool) {
    let min_x = cave.iter()
        .min_by(
            |(a, _), (b, _)| a.0.cmp(&b.0)
        ).unwrap().0.0;
    let max_x = cave.iter()
        .max_by(
            |(a, _), (b, _)| a.0.cmp(&b.0)
        ).unwrap().0.0;
    let abyss = cave.iter()
        .max_by(
            |(a, _), (b, _)| a.1.cmp(&b.1)
        ).unwrap().0.1  + if include_floor {
            2
        } else {
            0
        };

    for y in 0..=abyss {
        for x in min_x..=max_x {
            if include_floor && y == abyss - 1 {
                print!("#")
            } else {
                match cave.get(&(x, y)) {
                    Some(Fill::Rock) => print!("#"),
                    Some(Fill::Sand) => print!("o"),
                    None => {
                        if (x, y) == (500, 0) {
                            print!("+")
                        } else {
                            print!(".")
                        }
                    }
                }
            }
        }
        println!()
    }
}

fn check_sand_movement(
    cave: &HashMap<(usize, usize), Fill>,
    sand: &(usize, usize),
    abyss: usize,
    include_floor: bool
) -> Option<(usize, usize)> {
    if include_floor && sand.1 == abyss - 1 {
        None
    } else if !cave.contains_key(&(sand.0, sand.1 + 1)) {
        Some((sand.0, sand.1 + 1))
    } else if !cave.contains_key(&(sand.0 - 1, sand.1 + 1)) {
        Some((sand.0 - 1, sand.1 + 1))
    } else if !cave.contains_key(&(sand.0 + 1, sand.1 + 1)) {
        Some((sand.0 + 1, sand.1 + 1))
    } else {
        None
    }
}

fn sand_fill(cave: &str, include_floor: bool) -> u32 {
    let mut cave = build_cave(cave);
    let abyss = abyss_height(&cave) + if include_floor {
        2
    } else {
        0
    };

    let mut sand = (500, 0);
    loop {
        if sand.1 > abyss {
            break;
        } else {
            match check_sand_movement(&cave, &sand, abyss, include_floor) {
                Some(c) => sand = c,
                None => {
                    cave.insert(sand, Fill::Sand);
                    if sand == (500, 0) {
                        break;
                    }
                    sand = (500, 0);
                }
            }
        }
    }
    
    cave.iter()
        .filter(|(_, f)| matches!(f, Fill::Sand))
        .count() as u32
}

pub fn find_sand_fill_amount(input_path: &str, include_floor: bool) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => sand_fill(&content, include_floor),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP1: &str = 
r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn test_input1() {
        assert_eq!(sand_fill(TEST_INP1, false), 24)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(sand_fill(TEST_INP1, true), 93)
    }
}