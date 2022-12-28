mod nets;

#[derive(Debug)]
enum Command {
    Move(usize),
    RotateClock,
    RotateCouter
} 

#[derive(Debug, Clone)]
pub enum BoardTile {
    Void,
    Open,
    Wall
}

impl BoardTile {
    pub fn value(&self) -> u64 {
        match self {
            Self::Void => 0,
            Self::Open => 1,
            Self::Wall => 2
        }
    }

    fn is_void(&self) -> bool {
        match self {
            Self::Void => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Heading {
    North,
    South,
    East,
    West
}

impl Heading {
    pub fn value(&self) -> u64 {
        match self {
            Heading::North => 3,
            Heading::South => 1,
            Heading::East => 0,
            Heading::West => 2
        }
    }

    fn rotate_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }
}

fn to_position(flat: usize, board_size: usize) -> (usize, usize) {
    (flat % (board_size * 4), flat / (board_size * 4))
}

fn to_index((pos_x, pos_y): &(usize, usize), board_size: usize) -> usize {
    pos_x + pos_y * (board_size * 4)
}

#[allow(dead_code)]
fn print_board(
    board: &Vec<BoardTile>,
    pos: &(usize, usize),
    heading: &Heading,
    board_size: usize
) {
    let mut prev = 0;
    println!("===");
    board.iter().enumerate()
        .for_each(
            |(flat, b)| {
                let (x, y) = to_position(flat, board_size); 
                if prev != y {
                    println!();
                    prev = y;
                }
                if &(x, y) == pos {
                    match heading {
                        Heading::North => print!("^"),
                        Heading::South => print!("v"),
                        Heading::East => print!(">"),
                        Heading::West => print!("<"),
                    }
                } else {
                    match b {
                        BoardTile::Void => print!(" "),
                        BoardTile::Open => print!("."),
                        BoardTile::Wall => print!("#"),
                    }
                }
            }
        );
    println!();
}

fn build_board_and_commands(board: &str, board_size: usize) -> (Vec<BoardTile>, Vec<Command>) {
    let mut tiles = vec![BoardTile::Void; (board_size * 4).pow(2)];
    let mut commands = Vec::new();

    let lines: Vec<_> = board.lines().collect();
    if let [head @ .., tail] = lines.as_slice() {
        for (y, b) in head.into_iter().enumerate() {
            for (x, c) in b.chars().enumerate() {
                match c {
                    ' ' => tiles[to_index(&(x, y), board_size)] = BoardTile::Void,
                    '.' => tiles[to_index(&(x, y), board_size)] = BoardTile::Open,
                    '#' => tiles[to_index(&(x, y), board_size)] = BoardTile::Wall,
                    _ => panic!("Undefined board tile '{}'", c)
                };
            }
        }

        let mut mov = 0;
        for c in tail.chars() {
            match c {
                '0'..='9' => mov = mov * 10 + c.to_digit(10).unwrap() as usize,
                'R' => {
                    commands.push(Command::Move(mov));
                    mov = 0;
                    commands.push(Command::RotateClock);
                },
                'L' => {
                    commands.push(Command::Move(mov));
                    mov = 0;
                    commands.push(Command::RotateCouter);
                },
                _ => panic!("Unknown command '{}'", c)
            }
        }
        if mov != 0 {
            commands.push(Command::Move(mov));
        }
    } else {
        panic!("Input too short.")
    }
    (tiles, commands)
}

fn vertical_update(x: usize, y: usize, val: isize, board_size: usize) -> (usize, usize) {
    if y == 0 && val < 0 {
        (x, (board_size * 4) - 1)
    } else if y == (board_size * 4) - 1 && val > 0 {
        (x, 0)
    } else {
        (x, (y as isize + val) as usize)
    }
}

fn horizontal_update(x: usize, y: usize, val: isize, board_size: usize) -> (usize, usize) {
    if x == 0 && val < 0 {
        ((board_size * 4) - 1, y)
    } else if x == (board_size * 4) - 1 && val > 0 {
        (0, y)
    } else {
        ((x as isize + val) as usize, y)
    }
}

fn make_move(
    board: &Vec<BoardTile>,
    (mut pos_x, mut pos_y): &(usize, usize),
    heading: &Heading,
    mut mov: usize,
    board_size: usize
) -> (usize, usize) {
    let step: isize = if let Heading::North | Heading::West = heading {
        -1
    } else {
        1
    };
    let update_fn = match heading {
        Heading::North | Heading::South => {
            vertical_update
        },
        Heading::East | Heading::West => {
            horizontal_update
        }
    };
    while mov > 0 {
        let (px, py) = update_fn(pos_x, pos_y, step, board_size);
        (pos_x, pos_y) = match board[to_index(&(px, py), board_size)] {
            BoardTile::Void => {
                let (mut vx, mut vy) = (px, py); 
                loop {
                    (vx, vy) = match board[to_index(&(vx, vy), board_size)] {
                        BoardTile::Void => update_fn(vx, vy, step, board_size),
                        BoardTile::Open => {
                            mov -= 1;
                            break (vx, vy)
                        },
                        BoardTile::Wall => {
                            mov = 0;
                            break (pos_x, pos_y)
                        }
                    }
                }
            },
            BoardTile::Open => {
                mov -= 1;
                (px, py)
            },
            BoardTile::Wall => {
                mov = 0;
                (pos_x, pos_y)
            }
        }
    }
    (pos_x, pos_y)
}

fn trace(board: &str, board_size: usize) -> u64 {
    let (board, commands) = build_board_and_commands(&board, board_size);
        
    let flat = board.iter()
        .position(|b| !matches!(b, BoardTile::Void))
        .unwrap();
    let (mut pos_x, mut pos_y) = to_position(flat, board_size);
    let mut heading = Heading::East;

    for c in commands.into_iter() {
        match c {
            Command::Move(mov) => {
                (pos_x, pos_y) = make_move(
                    &board,
                    &(pos_x, pos_y),
                    &heading,
                    mov,
                    board_size
                );
            },
            Command::RotateClock => {
                heading = match &heading {
                    Heading::North => Heading::East,
                    Heading::South => Heading::West,
                    Heading::East => Heading::South,
                    Heading::West => Heading::North,
                }
            },
            Command::RotateCouter => {
                heading = match &heading {
                    Heading::North => Heading::West,
                    Heading::South => Heading::East,
                    Heading::East => Heading::North,
                    Heading::West => Heading::South,
                }
            }
        }
    }
    
    1000 * (pos_y + 1) as u64 + 4 * (pos_x + 1) as u64 + heading.value()
}

pub fn trace_path(input_path: &str) -> u64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => trace(&content, 50_usize),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn make_move_cube(
    board: &Vec<BoardTile>,
    (mut pos_x, mut pos_y): &(usize, usize),
    heading: &Heading,
    mut mov: usize,
    board_size: usize,
    net: &nets::Net
) -> ((usize, usize), Heading) {
    let mut mheading = *heading;
    while mov > 0 {
        let step: isize = if let Heading::North | Heading::West = mheading {
            -1
        } else {
            1
        };
        let (px, py) = match mheading {
            Heading::North | Heading::South => {
                vertical_update(pos_x, pos_y, step, board_size)
            },
            Heading::East | Heading::West => {
                horizontal_update(pos_x, pos_y, step, board_size)
            }
        };
        (pos_x, pos_y) = match board[to_index(&(px, py), board_size)] {
            BoardTile::Void => {
                let (npos, nheading) = net.move_to_adj_face(&(pos_x, pos_y), heading, board_size);
                match board[to_index(&npos, board_size)] {
                    BoardTile::Void => panic!("Moved into a void face"),
                    BoardTile::Wall => {
                        mov = 0;
                        (pos_x, pos_y)
                    },
                    BoardTile::Open => {
                        mov -= 1;
                        mheading = nheading;
                        npos
                    }
                }
            },
            BoardTile::Open => {
                mov -= 1;
                (px, py)
            },
            BoardTile::Wall => {
                mov = 0;
                (pos_x, pos_y)
            }
        }
    }
    ((pos_x, pos_y), mheading)
}

fn trace_cube(board: &str, board_size: usize) -> u64 {
    let (board, commands) = build_board_and_commands(&board, board_size);
    let net = nets::Net::find_net(&board, board_size);
        
    let first_open = board.iter()
        .position(|b| !matches!(b, BoardTile::Void))
        .unwrap();
    let (mut pos_x, mut pos_y) = to_position(first_open, board_size);
    let mut heading = Heading::East;

    for c in commands.into_iter() {
        match c {
            Command::Move(mov) => {
                ((pos_x, pos_y), heading) = make_move_cube(
                    &board,
                    &(pos_x, pos_y),
                    &heading,
                    mov,
                    board_size,
                    &net
                );
            },
            Command::RotateClock => {
                heading = heading.rotate_clockwise()
            },
            Command::RotateCouter => {
                heading = heading.rotate_counterclockwise()
            }
        }
    }
    
    1000 * (pos_y + 1) as u64 + 4 * (pos_x + 1) as u64 + heading.value()
}

pub fn trace_path_on_cube(input_path: &str) -> u64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => trace_cube(&content, 50_usize),
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
r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

    #[test]
    fn test_input1() {
        assert_eq!(trace(TEST_INP1, 4_usize), 6032)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(trace_cube(TEST_INP1, 4_usize), 5031)
    }
}