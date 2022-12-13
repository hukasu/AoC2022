#[derive(Debug)]
enum Instruction {
    AddX(isize),
    NoOp
}

pub fn default_probe(clock_counter: usize) -> bool {
    (clock_counter + 20) % 40 == 0
}

fn compile_program(program_code: &str) -> Vec<Instruction> {
    program_code.lines()
        .map(
            |l| {
                match Vec::from_iter(l.split_whitespace()).as_slice() {
                    ["addx", x] => Instruction::AddX(x.parse().unwrap()),
                    ["noop"] => Instruction::NoOp,
                    _ => panic!("Unknown instruction '{}'", l)
                }
            }
        )
        .collect()
}

fn execute_prober(program_code: &str, probe_condition: fn(usize) -> bool) -> i32 {
    let program = compile_program(program_code);
    program.iter()
        .fold(
            (1, 1, 0),
        |(clock, reg_x, probe), inst| {
            match inst {
                Instruction::AddX(x) => {
                    let p = if probe_condition(clock) {
                        reg_x * (clock as isize)
                    } else if probe_condition(clock + 1) {
                        reg_x * ((clock + 1) as isize)
                    } else {
                        0
                    };
                    (clock + 2, reg_x + x, probe + p)
                },
                Instruction::NoOp => {
                    let p = if probe_condition(clock) {
                        reg_x * (clock as isize)
                    } else {
                        0
                    };
                    (clock + 1, reg_x, probe + p)
                }
            }
        }
    ).2 as i32
}

pub fn decode_cpu_clock(input_path: &str, probe_condition: fn(usize) -> bool) -> i32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => execute_prober(&content, probe_condition),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn render(program_code: &str) -> String {
    let program = compile_program(program_code);
    let mut program_counter = program.iter().peekable();
    (0..(40 * 6)).into_iter()
        .fold(
            (String::new(), &mut program_counter, 1_isize, false),
            |(screen, pc, reg_x, add_cycle), clock| {
                let x_pos = clock % 40;
                let n_screen = if x_pos == 0 && clock != 0 {
                    screen + "\n"
                } else {
                    screen
                };
                let n_screen = if (x_pos == reg_x - 1) || (x_pos == reg_x) || (x_pos == reg_x + 1) {
                    n_screen + "#"
                } else {
                    n_screen + "."
                };
                let cur_inst = pc.peek().unwrap();
                match (cur_inst, add_cycle) {
                    (Instruction::AddX(x), true) => {
                        pc.next();
                        (n_screen, pc, reg_x + x, false)
                    },
                    (Instruction::AddX(_x), false) => {
                        (n_screen, pc, reg_x, true)
                    },
                    (Instruction::NoOp, false) => {
                        pc.next();
                        (n_screen, pc, reg_x, false)
                    },
                    _ => panic!("CPU failure. '{:?} ({})' at {}", cur_inst, add_cycle, clock)
                }
            }
        ).0
}

pub fn render_crt(input_path: &str) -> String {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => render(&content),
        Err(er) => {
            println!("{}", er);
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP1: &str = 
r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    const TEST_RENDER: &str =
r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

    #[test]
    fn test_input1() {
        assert_eq!(execute_prober(TEST_INP1, default_probe), 13140)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(render(TEST_INP1), TEST_RENDER)
    }
}