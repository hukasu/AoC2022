use std::collections::{BTreeSet, BTreeMap};

#[allow(dead_code)]
fn print_positions(positions: &BTreeSet<(isize, isize)>) {
    let pos_bounds = position_bounds(positions);
    // println!("{:?}", pos_bounds);
    (pos_bounds.0.1..=pos_bounds.1.1).rev().for_each(
        |y| {
            (pos_bounds.0.0..=pos_bounds.1.0).for_each(
                |x| {
                    if positions.contains(&(x, y)) {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
            );
            println!();
        }
    )
}

fn position_bounds(positions: &BTreeSet<(isize, isize)>) -> ((isize, isize), (isize, isize)) {
    (
        (
            positions.iter().map(|(x, _)| *x).min_by(|lx, rx| lx.cmp(rx)).unwrap(),
            positions.iter().map(|(_, y)| *y).min_by(|ly, ry| ly.cmp(ry)).unwrap()
        ),
        (
            positions.iter().map(|(x, _)| *x).max_by(|lx, rx| lx.cmp(rx)).unwrap(),
            positions.iter().map(|(_, y)| *y).max_by(|ly, ry| ly.cmp(ry)).unwrap()
        )
)
}

fn initial_positions(plots: &str) -> BTreeSet<(isize, isize)> {
    let mut bt = BTreeSet::new();
    plots.lines().rev().enumerate()
        .for_each(
            |(y, l)| {
                l.chars().enumerate()
                    .for_each(
                        |(x, c)| {
                            if c == '#' {
                                bt.insert((x as isize, y as isize));
                            }
                        }
                    )
            }
        );
    bt
}

fn check_north(pos: &(isize, isize), positions: &BTreeSet<(isize, isize)>) -> bool {
    let nw = !positions.contains(&(pos.0 - 1, pos.1 + 1));
    let n = !positions.contains(&(pos.0, pos.1 + 1));
    let ne = !positions.contains(&(pos.0 + 1, pos.1 + 1));
    nw && n && ne
}

fn check_south(pos: &(isize, isize), positions: &BTreeSet<(isize, isize)>) -> bool {
    let sw = !positions.contains(&(pos.0 - 1, pos.1 - 1));
    let s = !positions.contains(&(pos.0, pos.1 - 1));
    let se = !positions.contains(&(pos.0 + 1, pos.1 - 1));
    sw && s && se
}

fn check_east(pos: &(isize, isize), positions: &BTreeSet<(isize, isize)>) -> bool {
    let ne = !positions.contains(&(pos.0 + 1, pos.1 + 1));
    let e = !positions.contains(&(pos.0 + 1, pos.1));
    let se = !positions.contains(&(pos.0 + 1, pos.1 - 1));
    ne && e && se
}

fn check_west(pos: &(isize, isize), positions: &BTreeSet<(isize, isize)>) -> bool {
    let nw = !positions.contains(&(pos.0 - 1, pos.1 + 1));
    let w = !positions.contains(&(pos.0 - 1, pos.1));
    let sw = !positions.contains(&(pos.0 - 1, pos.1 - 1));
    nw && w && sw
}

fn round(positions: &BTreeSet<(isize, isize)>, round: usize) -> BTreeSet<(isize, isize)> {
    let intents: BTreeMap<_, _> = positions.iter()
        .map(
            |pos| {
                let round_tests = vec![
                    (check_north(pos, positions), (pos.0, pos.1 + 1)),
                    (check_south(pos, positions), (pos.0, pos.1 - 1)),
                    (check_west(pos, positions), (pos.0 - 1, pos.1)),
                    (check_east(pos, positions), (pos.0 + 1, pos.1))
                ];
                if round_tests.iter().all(|(b, _)| *b) {
                    (pos, *pos)
                } else {
                    let round_cycle = round_tests.iter().cycle();
                    let mut round_intent = None;
                    for (test, intent) in round_cycle.skip(round).take(4) {
                        if *test {
                            round_intent = Some(intent);
                            break;
                        }
                    }
                    let fround_intent = round_intent.unwrap_or(pos);
                    (pos, *fround_intent)
                }
            }
        )
        .collect();
    let unique_intents = intents.values()
        .fold(
            (BTreeSet::new(), BTreeSet::new()),
            |(mut uniq, mut dup), int| {
                if dup.contains(int) {
                    ()
                } else if uniq.contains(int) {
                    uniq.remove(int);
                    dup.insert(int);   
                } else {
                    uniq.insert(int);
                };
                (uniq, dup)
            }
        ).0;
        
    intents.iter()
        .filter_map(
            |(pos, int)| {
                if unique_intents.contains(&int) {
                    Some(*int)
                } else {
                    Some(**pos)
                }
            } 
        )
        .collect()
}

fn empty_plots(positions: &BTreeSet<(isize, isize)>) -> u64 {
    let (minbounds, maxbounds) = position_bounds(positions);
    (minbounds.0..=maxbounds.0).fold(
        0,
        |cum, x| {
            cum + (minbounds.1..=maxbounds.1).fold(
                0,
                |ycum, y| {
                    if positions.contains(&(x, y)) {
                        ycum
                    } else {
                        ycum + 1
                    }
                }
            )
        }
    )
}

fn game_of_plating(plots: &str, rounds: usize) -> (u64, u64) {
    let mut positions = initial_positions(plots);
    let mut stable_round = 0;
    for r in 0..rounds {
        let tmp = round(&positions, r);
        stable_round = r + 1;
        if tmp.eq(&positions) {
            break;
        } else {
            positions = tmp;
        }
    }
    (empty_plots(&positions), stable_round as u64)
}

pub fn elfs_game_of_plating(input_path: &str) -> u64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => game_of_plating(&content, 10).0,
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

pub fn elfs_game_of_plating_till_stable(input_path: &str) -> u64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => game_of_plating(&content, std::usize::MAX).1,
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
r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;

    #[test]
    fn test_input1() {
        assert_eq!(game_of_plating(TEST_INP1, 10).0, 110)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(game_of_plating(TEST_INP1, std::usize::MAX).1, 20)
    }
}