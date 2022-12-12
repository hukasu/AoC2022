#[derive(Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right
}

fn build_move_list(knot_moves: &str) -> Vec<Move> {
    knot_moves.lines()
        .map(
            |l| {
                let (dir, count) = l.split_once(" ").unwrap();
                let m = match dir {
                    "U" => Move::Up,
                    "D" => Move::Down,
                    "L" => Move::Left,
                    "R" => Move::Right,
                    _ => panic!("Unknown move,")
                };
                vec![m; count.parse().unwrap()]
            }
        )
        .flatten()
        .collect()
}

fn move_head(head: &(i32, i32), m: &Move) -> (i32, i32) {
    match m {
        Move::Up => (head.0, head.1 + 1),
        Move::Down => (head.0, head.1 - 1),
        Move::Left => (head.0 - 1, head.1),
        Move::Right => (head.0 + 1, head.1)
    }
}

fn move_knot(parent: &(i32, i32), current: &(i32, i32)) -> (i32, i32) {
    match ((parent.0 - current.0), (parent.1 - current.1)) {
        (-1..=1, -1..=1) => *current,
        ( 2,       0) => (current.0 + 1, current.1    ),
        ( 2,  1..= 2) => (current.0 + 1, current.1 + 1),
        ( 2, -2..=-1) => (current.0 + 1, current.1 - 1),
        (-2,       0) => (current.0 - 1, current.1    ),
        (-2,  1..= 2) => (current.0 - 1, current.1 + 1),
        (-2, -2..=-1) => (current.0 - 1, current.1 - 1),
        ( 0,       2) => (current.0    , current.1 + 1),
        ( 1..= 2,  2) => (current.0 + 1, current.1 + 1),
        (-2..=-1,  2) => (current.0 - 1, current.1 + 1),
        ( 0,      -2) => (current.0    , current.1 - 1),
        ( 1..= 2, -2) => (current.0 + 1, current.1 - 1),
        (-2..=-1, -2) => (current.0 - 1, current.1 - 1),
        _ => panic!("Distance too far {:?} {:?}", parent, current)
    }
}

fn run_knot_motion_simulation(knot_moves: &str, rope_length: usize) -> u32 {
    let moves = build_move_list(knot_moves);
    moves.iter()
        .fold(
            (&mut std::collections::BTreeSet::new(), &mut vec![(0, 0); rope_length]),
            |(set, knots), m| {
                let mut knots_iter = knots.iter_mut();
                let mut previous = knots_iter.next().unwrap();
                *previous = move_head(previous, m);
                for knot in knots_iter {
                    *knot = move_knot(previous, &knot);
                    previous = knot;
                }
                set.insert(*previous);
                (set, knots)
            }
        )
        .0
        .len() as u32
}

pub fn simulate_knot_motion(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => run_knot_motion_simulation(&content, 2),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

pub fn simulate_long_knot_motion(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => run_knot_motion_simulation(&content, 10),
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
r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

const TEST_INP2: &str = 
r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn test_input1() {
        assert_eq!(run_knot_motion_simulation(TEST_INP1, 2), 13)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(run_knot_motion_simulation(TEST_INP2, 10), 36)
    }
}