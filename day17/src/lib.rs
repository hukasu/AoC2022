use std::{iter::{Cycle, Enumerate, Peekable}, str::{Chars}, collections::{BTreeMap, VecDeque, HashMap}};

const PATTERN_WINDOW: usize = 8;

#[derive(Debug)]
enum Rock {
    Flat,
    Tall,
    Plus,
    L,
    Square
}

impl Rock {
    fn shape(&self, origin: &(usize, usize)) -> Vec<(usize, usize)> {
        match self {
            Rock::Flat => vec![
                (origin.0    , origin.1),
                (origin.0 + 1, origin.1),
                (origin.0 + 2, origin.1),
                (origin.0 + 3, origin.1),
            ],
            Rock::Tall => vec![
                (origin.0, origin.1    ),
                (origin.0, origin.1 + 1),
                (origin.0, origin.1 + 2),
                (origin.0, origin.1 + 3),
            ],
            Rock::Plus => vec![
                (origin.0    , origin.1 + 1),
                (origin.0 + 1, origin.1    ),
                (origin.0 + 1, origin.1 + 1),
                (origin.0 + 2, origin.1 + 1),
                (origin.0 + 1, origin.1 + 2),
            ],
            Rock::L => vec![
                (origin.0    , origin.1    ),
                (origin.0 + 1, origin.1    ),
                (origin.0 + 2, origin.1    ),
                (origin.0 + 2, origin.1 + 1),
                (origin.0 + 2, origin.1 + 2),
            ],
            Rock::Square => vec![
                (origin.0    , origin.1    ),
                (origin.0    , origin.1 + 1),
                (origin.0 + 1, origin.1    ),
                (origin.0 + 1, origin.1 + 1),
            ]
        }
    }
}

#[allow(dead_code)]
fn print_shaft(shaft: &BTreeMap<usize, [bool; 7]>) {
    shaft.iter().rev()
        .for_each(
            |(_, row)| {
                print!("|");
                row.iter()
                    .for_each(
                        |b| {
                            if *b {
                                print!("#")
                            } else {
                                print!(".")
                            }
                        }
                    );
                println!("|");
            }
        );
    println!("+-------+");
}

fn collision(
    rock: &Rock,
    position: &(usize, usize),
    shaft: &BTreeMap<usize, [bool; 7]>
) -> bool {
    let shape = rock.shape(position);
    shape.into_iter()
        .map(
            |(x, y)| {
                if x >= 7 {
                    true
                } else {
                    shaft.get(&y)
                        .and_then(
                            |row| Some(
                                row.get(x)
                                    .map(
                                        |b| *b
                                    )
                                    .unwrap_or(false))
                        )
                        .unwrap_or(false)
                }
            }
        )
        .any(|b| b)
}

fn try_jet(jet: &char, position: &(usize, usize)) -> (usize, usize) {
    match (jet, position.0) {
        ('>', _) => (position.0 + 1, position.1),
        ('<', 0) => (position.0, position.1),
        ('<', _) => (position.0 - 1, position.1),
        _ => panic!("Unresolved jet '{}' at {:?}", jet, position)
    }
}

fn try_fall(position: &(usize, usize)) -> (usize, usize) {
    match position.1 {
        0 => position.clone(),
        _ => (position.0, position.1 - 1)
    }
}

fn tallest_point(shaft: &BTreeMap<usize, [bool; 7]>) -> usize {
    shaft.keys().map(|k| *k + 1).max().unwrap_or(0)
}

fn resolve_rock(rock: &Rock, jets: &mut Peekable<Cycle<Enumerate<Chars>>>, shaft: &mut BTreeMap<usize, [bool; 7]>) {
    const ROLLING_WINDOW: usize = 128;
    let mut position = (2, tallest_point(shaft) + 3);
    for (_, jet) in jets {
        let pushed = try_jet(&jet, &position);
        position = if collision(&rock, &pushed, shaft) {
            position
        } else {
            pushed
        };
        let fallen = try_fall(&position);
        if collision(&rock, &fallen, shaft) || position.1 == 0 {
            let shape = rock.shape(&position);
            shape.into_iter()
                .for_each(
                    |(x, y)| {
                        if !shaft.contains_key(&y) {
                            shaft.insert(y, [false; 7]);
                            if y >= ROLLING_WINDOW {
                                shaft.remove(&(y - ROLLING_WINDOW));
                            }
                        }
                        let row = shaft.get_mut(&y).unwrap();
                        row[x] = true;
                    }
                );
            break;
        } else {
            position = fallen;
        };
    };
}

fn falling_rocks(jets: &str, stop_at: usize) -> u128 {
    let rocks = vec![Rock::Flat, Rock::Plus, Rock::L, Rock::Tall, Rock::Square];
    let mut rocks_iter = rocks.iter().cycle().enumerate();
    let mut jets = jets.chars().enumerate().cycle().peekable();
    
    let mut shaft = BTreeMap::new();
    
    let mut cur = VecDeque::new();
    let mut repeat = HashMap::new();
    while !repeat.contains_key(&cur) {
        let (r_ind, rock) = rocks_iter.next().unwrap();
        let starting_jet = jets.peek().unwrap().0;
        if r_ind == stop_at {
            break;
        }
        resolve_rock(rock, &mut jets, &mut shaft);

        repeat.insert(cur.clone(), (r_ind, starting_jet, tallest_point(&shaft) as u128));
        
        if cur.len() == PATTERN_WINDOW {
            cur.pop_front();

        }
        cur.push_back(starting_jet);
    }
    // print_shaft(&shaft);
    // println!("Repeats {:?}", repeat);
    // println!("Cur ({}) {:?}", cur.len(), cur);
    let first_pattern = repeat.get(&cur).unwrap().0 - 1;
    // println!("First pattern {:?}", first_pattern);
    let first_pattern_unbiased = first_pattern - PATTERN_WINDOW;
    // println!("First pattern unbiased {:?}", first_pattern_unbiased);
    let pattern_length = repeat.len() - first_pattern - 1;
    // println!("pattern length {:?}", pattern_length);
    let heights = repeat.iter()
        .fold(
            HashMap::new(),
            |mut hm, (_, (r, _, he))| {
                hm.insert(*r, *he);
                hm
            }
        );
    let head = heights[&first_pattern_unbiased];
    let pattern = (heights[&(first_pattern_unbiased + pattern_length)] - head) * ((stop_at - first_pattern_unbiased) / pattern_length) as u128;
    let tail = heights[&(first_pattern_unbiased - 1 + (stop_at - first_pattern_unbiased) % pattern_length)];
    pattern + tail
}

pub fn falling_rocks_tower(input_path: &str, stop_at: usize) -> u128 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => falling_rocks(&content, stop_at),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

// pub fn release_pressure_with_help(input_path: &str) -> u32 {
//     let content = std::fs::read_to_string(input_path);
//     match content {
//         Ok(content) => depressurize_with_help(&content),
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
r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn test_input1() {
        assert_eq!(falling_rocks(TEST_INP1, 2022), 3068)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(falling_rocks(TEST_INP1, 1_000_000_000_000), 1514285714288)
    }
}