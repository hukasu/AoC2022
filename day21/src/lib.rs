use std::collections::HashMap;

enum Operation {
    Const(f64),
    Add(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
}

fn build_operations(code: &str) -> HashMap<String, Operation> {
    code.lines()
        .fold(
            HashMap::new(),
            |mut hm, l| {
                let spl: Vec<_> = l.split(' ').collect();
                match spl[1..] {
                    [a] => hm.insert(
                        spl[0].trim_end_matches(':').to_string(),
                        Operation::Const(a.parse().unwrap())
                    ),
                    [l, "+", r] => hm.insert(
                        spl[0].trim_end_matches(':').to_string(),
                        Operation::Add(l.to_string(), r.to_string())
                    ),
                    [l, "-", r] => hm.insert(
                        spl[0].trim_end_matches(':').to_string(),
                        Operation::Sub(l.to_string(), r.to_string())
                    ),
                    [l, "*", r] => hm.insert(
                        spl[0].trim_end_matches(':').to_string(),
                        Operation::Mult(l.to_string(), r.to_string())
                    ),
                    [l, "/", r] => hm.insert(
                        spl[0].trim_end_matches(':').to_string(),
                        Operation::Div(l.to_string(), r.to_string())
                    ),
                    _ => panic!("Unresolved operation '{}'", l)
                };
                hm
            }
        )
}

fn do_math(operations: &HashMap<String, Operation>, monkey: &String) -> f64 {
    let op = operations.get(monkey).unwrap();
    match op {
        Operation::Const(a) => *a,
        Operation::Add(l, r) => do_math(operations, l) + do_math(operations, r),
        Operation::Sub(l, r) => do_math(operations, l) - do_math(operations, r),
        Operation::Mult(l, r) => do_math(operations, l) * do_math(operations, r),
        Operation::Div(l, r) => do_math(operations, l) / do_math(operations, r),
    }
}

fn monkey_math(code: &str) -> i64 {
    let math = build_operations(&code);
    do_math(&math, &String::from("root")) as i64
}

pub fn execute_monkey_math(input_path: &str) -> i64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => monkey_math(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn unwrap_operation(operations: &HashMap<String, Operation>, monkey: &String) -> (f64, f64) {
    match operations.get(monkey).unwrap() {
        Operation::Const(a) => (*a, *a),
        Operation::Add(l, r) => {
            (
                do_math(&operations, l),
                do_math(&operations, r)
            )
        },
        Operation::Sub(l, r) => {
            (
                do_math(&operations, l),
                do_math(&operations, r)
            )
        },
        Operation::Mult(l, r) => {
            (
                do_math(&operations, l),
                do_math(&operations, r)
            )
        },
        Operation::Div(l, r) => {
            (
                do_math(&operations, l),
                do_math(&operations, r)
            )
        }
    }
}

fn update_human(operations: &mut HashMap<String, Operation>, step: f64) {
    if let Operation::Const(human) = operations[&String::from("humn")] {
        operations.insert(String::from("humn"), Operation::Const(human + step))
    } else {
        panic!("Failed to unwrap 'humn'");
    };
}

fn monkey_riddle(code: &str) -> i64 {
    let mut math = build_operations(&code);

    loop {
        let a_x = do_math(&math, &String::from("humn"));
        let (a_yc1, a_yc2) = unwrap_operation(&math, &String::from("root"));
        if (a_yc1 - a_yc2).abs() < std::f64::EPSILON {
            break a_x as i64;
        }
    
        update_human(&mut math, -a_x);
        let (b_yc1, b_yc2) = unwrap_operation(&math, &String::from("root"));
    
        let (a_y, b_y, y) = if a_yc1 == b_yc1 {
            (a_yc2, b_yc2, b_yc1)
        } else {
            (a_yc1, b_yc1, b_yc2)
        };
        
        let c = b_y;
        let slope = (a_y - b_y) / a_x;
    
        let res = (y - c) / slope;
        update_human(&mut math, res);
    }
}

pub fn solve_monkey_riddle(input_path: &str) -> i64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => monkey_riddle(&content),
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
r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#;

    #[test]
    fn test_input1() {
        assert_eq!(monkey_math(TEST_INP1), 152)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(monkey_riddle(TEST_INP1), 301)
    }
}