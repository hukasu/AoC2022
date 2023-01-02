use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoardTile {
    Wall,
    // North, South, East, West
    Blizzards(bool, bool, bool, bool)
}

impl std::fmt::Display for BoardTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wall => '#',
                Self::Blizzards(false, false, false, false) => '.',
                Self::Blizzards(true, false, false, false) => '^',
                Self::Blizzards(false, true, false, false) => 'v',
                Self::Blizzards(false, false, true, false) => '>',
                Self::Blizzards(false, false, false, true) => '<',
                Self::Blizzards(_, _, _, _) => 'N'
            }
        )
    }
}

#[allow(dead_code)]
fn print_board(board: &Vec<Vec<BoardTile>>) {
    board.iter()
        .for_each(
            |row| {
                row.iter()
                    .for_each(
                        |t| print!("{}", t)
                    );
                println!();
            }
        )
}

fn build_board(board: &str) -> Vec<Vec<BoardTile>> {
    board.lines()
        .map(
            |l| {
                l.chars()
                    .map(
                        |c| {
                            match c {
                                '#' => BoardTile::Wall,
                                '.' => BoardTile::Blizzards(false, false, false, false),
                                '^' => BoardTile::Blizzards(true, false, false, false),
                                'v' => BoardTile::Blizzards(false, true, false, false),
                                '>' => BoardTile::Blizzards(false, false, true, false),
                                '<' => BoardTile::Blizzards(false, false, false, true),
                                _ => panic!("Unknown tile type '{}'", c)
                            }
                        }
                    )
                    .collect()
            }
        )
        .collect()
}

fn resolve_blizzards(board: &Vec<Vec<BoardTile>>) -> Vec<Vec<BoardTile>> {
    board.iter().enumerate()
        .map(
            |(y, row)| {
                if y > 0 && y < board.len() - 1 {
                    row.iter().enumerate()
                        .map(
                            |(x, tile)| {
                                if x > 0 && x < board[y].len() - 1 {
                                    let north_test = if y == 1 {
                                        board[board.len() - 2][x]
                                    } else {
                                        board[y - 1][x]
                                    };
                                    let from_north = matches!(north_test, BoardTile::Blizzards(_, true, _, _));
                                    let south_test = if y == board.len() - 2 {
                                        board[1][x]
                                    } else {
                                        board[y + 1][x]
                                    };
                                    let from_south = matches!(south_test, BoardTile::Blizzards(true, _, _, _));
                                    let east_test = if x == board[y].len() - 2 {
                                        board[y][1]
                                    } else {
                                        board[y][x + 1]
                                    };
                                    let from_east = matches!(east_test, BoardTile::Blizzards(_, _, _, true));
                                    let west_test = if x == 1 {
                                        board[y][board[y].len() - 2]
                                    } else {
                                        board[y][x - 1]
                                    };
                                    let from_west = matches!(west_test, BoardTile::Blizzards(_, _, true, _));
                                    BoardTile::Blizzards(from_south, from_north, from_west, from_east)
                                } else {
                                    tile.clone()
                                }
                            }
                        )
                        .collect()
                } else {
                    row.clone()
                }
            }
        )
        .collect()
}

fn resolve_elves(
    board: &Vec<Vec<BoardTile>>,
    positions: &BTreeSet<(usize, usize)>
) -> BTreeSet<(usize, usize)> {
    let mut next_pos = BTreeSet::new();
    positions.iter()
        .for_each(
            |(y, x)| {
                if matches!(board[*y][*x], BoardTile::Blizzards(false, false, false, false)) {
                    next_pos.insert((*y, *x));
                }
                if *y > 0 && matches!(board[y - 1][*x], BoardTile::Blizzards(false, false, false, false)) {
                    next_pos.insert((y - 1, *x));
                }
                if *y < board.len() - 1 && matches!(board[y + 1][*x], BoardTile::Blizzards(false, false, false, false)) {
                    next_pos.insert((y + 1, *x));
                }
                if *x > 0 && matches!(board[*y][x - 1], BoardTile::Blizzards(false, false, false, false)) {
                    next_pos.insert((*y, x - 1));
                }
                if *x < board[*y].len() - 1 &&  matches!(board[*y][x + 1], BoardTile::Blizzards(false, false, false, false)) {
                    next_pos.insert((*y, x + 1));
                }
            }
        );
    next_pos
}

fn trip(
    mut board: Vec<Vec<BoardTile>>,
    start_position: (usize, usize),
    target: (usize, usize)
) -> (Vec<Vec<BoardTile>>, u64) {
    let mut possible_pos = BTreeSet::from_iter(vec![start_position]);
    let mut elapsed = 0;

    while !possible_pos.contains(&target) {
        if elapsed >= 1000 {
            break;
        }
        elapsed += 1;
        board = resolve_blizzards(&board);
        possible_pos = resolve_elves(&board, &possible_pos);
    }
    (board, elapsed)
}

fn blizzard_pathing(board: &str) -> u64 {
    let board = build_board(board);
    let target = (board.len() - 1, board[0].len() - 2);
    trip(board, (0, 1), target).1
}

pub fn blizzard_dodging(input_path: &str) -> u64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => blizzard_pathing(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn blizzard_pathing_2_trips(board: &str) -> u64 {
    let board = build_board(board);
    let start = (0, 1);
    let target = (board.len() - 1, board[0].len() - 2);
    let (board, trip1) = trip(board, start, target);
    let (board, trip2) = trip(board, target, start);
    let (_, trip3) = trip(board, start, target);
    trip1 + trip2 + trip3
}

pub fn blizzard_dodging_2_trips(input_path: &str) -> u64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => blizzard_pathing_2_trips(&content),
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
r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

    #[test]
    fn test_input1() {
        assert_eq!(blizzard_pathing(TEST_INP1), 18)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(blizzard_pathing_2_trips(TEST_INP1), 54)
    }

    #[test]
    fn resolve_blizzards_1() {
        let blizzards = build_board(
r#"#####
#...#
#.^.#
#...#
#####"#
        );
        let expected = build_board(
r#"#####
#.^.#
#...#
#...#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_2() {
        let blizzards = build_board(
r#"#####
#...#
#.v.#
#...#
#####"#
        );
        let expected = build_board(
r#"#####
#...#
#...#
#.v.#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_3() {
        let blizzards = build_board(
r#"#####
#...#
#.>.#
#...#
#####"#
        );
        let expected = build_board(
r#"#####
#...#
#..>#
#...#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_4() {
        let blizzards = build_board(
r#"#####
#...#
#.<.#
#...#
#####"#
        );
        let expected = build_board(
r#"#####
#...#
#<..#
#...#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_5() {
        let blizzards = build_board(
r#"#####
#^^^#
#...#
#...#
#####"#
        );
        let expected = build_board(
r#"#####
#...#
#...#
#^^^#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_6() {
        let blizzards = build_board(
r#"#####
#...#
#...#
#vvv#
#####"#
        );
        let expected = build_board(
r#"#####
#vvv#
#...#
#...#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_7() {
        let blizzards = build_board(
r#"#####
#..>#
#..>#
#..>#
#####"#
        );
        let expected = build_board(
r#"#####
#>..#
#>..#
#>..#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_8() {
        let blizzards = build_board(
r#"#####
#<..#
#<..#
#<..#
#####"#
        );
        let expected = build_board(
r#"#####
#..<#
#..<#
#..<#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_9() {
        let blizzards = build_board(
r#"#####
#.><#
#.><#
#.><#
#####"#
        );
        let expected = build_board(
r#"#####
#.<>#
#.<>#
#.<>#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_10() {
        let blizzards = build_board(
r#"#####
#.>v#
#.^<#
#...#
#####"#
        );
        let expected = build_board(
r#"#####
#.^>#
#.<v#
#...#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_11() {
        let blizzards = build_board(
r#"#####
#<<<#
#<<<#
#<<<#
#####"#
        );
        let expected = build_board(
r#"#####
#<<<#
#<<<#
#<<<#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }

    #[test]
    fn resolve_blizzards_12() {
        let blizzards = build_board(
r#"#####
#>>>#
#>>>#
#>>>#
#####"#
        );
        let expected = build_board(
r#"#####
#>>>#
#>>>#
#>>>#
#####"#
        );
        assert!(resolve_blizzards(&blizzards).eq(&expected));
    }
}