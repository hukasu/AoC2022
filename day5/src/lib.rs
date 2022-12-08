#[derive(Clone)]
struct Crate(char);
#[derive(Clone)]
struct Cargo(Vec<Vec<Crate>>);
#[derive(Clone)]
struct Move(usize, usize, usize);

fn process_crate(line: &[char]) -> Option<Crate> {
    match line {
        ['[', c, ']', ' '] => Some(Crate(*c)),
        [' ', ' ', ' ', ' '] => None,
        ['[', c, ']'] => Some(Crate(*c)),
        [' ', ' ', ' '] => None,
        c => {
            panic!("Cargo line malformed '{c:?}'")
        }
    }
}

fn process_move(line: &str) -> Move {
    let l = Vec::from_iter(line.split_whitespace());
    let m: &[&str] = l.as_slice();
    if let ["move", n, "from", s, "to", d] = m {
        Move(n.parse().unwrap(), s.parse().unwrap(), d.parse().unwrap())
    } else {
        panic!("Malformed move '{:?}'", m)
    }
}

fn process_input(input: &str) -> (Cargo, Vec<Move>) {
    let mut lines = input.split_terminator("\n");
    let mut cargo_rows: Vec<Vec<Option<Crate>>> = vec![]; 
    for line in &mut lines {
        let chars = Vec::from_iter(line.chars());
        if let [' ', '1', ' ', ' '] = chars.chunks(4).next().unwrap() {
            break
        } else {
            cargo_rows.push(
                chars.chunks(4).map(process_crate).collect()
            );
        }
    }

    let mut cargo_columns = vec![vec![]; cargo_rows[0].len()];
    for row in cargo_rows.into_iter().rev() {
        for (i, c) in row.into_iter().enumerate() {
            if let Some(crat) = c {
                cargo_columns[i].push(crat);
            } 
        }
    }
    let cargo = Cargo(cargo_columns);

    lines.next();
    (cargo, lines.map(process_move).collect())
}

fn execute_moves(cargo: &mut Cargo, moves: &Vec<Move>) {
    for mov in moves {
        let Move(n, s, d) = mov;
        for _ in 0..*n {
            let tmp = cargo.0[*s - 1].pop().unwrap();
            cargo.0[*d - 1].push(tmp);
        }
    }
}

fn rearrenge_cargo(contents: &str) -> String {
    let (mut cargo, moves) = process_input(contents);
    execute_moves(&mut cargo, &moves);
    cargo.0.iter()
        .map(|v| v.last().unwrap().0.to_string())
        .fold(String::from(""), |sum, c| sum + c.as_str())
}

pub fn organize_cargo(input_path: &str) -> String {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => rearrenge_cargo(&content),
        Err(er) => {
            println!("{}", er);
            String::new()
        }
    }
}

fn execute_moves_9001(cargo: &mut Cargo, moves: &Vec<Move>) {
    for mov in moves {
        let Move(n, s, d) = mov;
        let mut q = vec![];
        for _ in 0..*n {
            q.push(cargo.0[*s - 1].pop().unwrap());
        }
        for c in q.into_iter().rev() {
            cargo.0[*d - 1].push(c);
        }
    }
}

fn rearrenge_cargo_9001(contents: &str) -> String {
    let (mut cargo, moves) = process_input(contents);
    execute_moves_9001(&mut cargo, &moves);
    cargo.0.iter()
        .map(|v| v.last().unwrap().0.to_string())
        .fold(String::from(""), |sum, c| sum + c.as_str())
}

pub fn organize_cargo_9001(input_path: &str) -> String {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => rearrenge_cargo_9001(&content),
        Err(er) => {
            println!("{}", er);
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP: &str =
r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_input() {
        assert_eq!(rearrenge_cargo(TEST_INP), "CMZ")
    }

    #[test]
    fn test_input_part2() {
        assert_eq!(rearrenge_cargo_9001(TEST_INP), "MCD")
    }
}