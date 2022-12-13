use std::cell::RefCell;

#[derive(Debug)]
enum Operation {
    AddI(i64),
    MultI(i64),
    Pow2,
}

impl Operation {
    fn exec(&self, rhs: i64) -> i64 {
        match self {
            Operation::AddI(lhs) => rhs + lhs,
            Operation::MultI(lhs) => rhs * lhs,
            Operation::Pow2 => rhs * rhs
        }
    }
}

#[derive(Debug)]
struct Test(i64, usize, usize);

impl Test {
    fn throw(&self, item: i64) -> usize {
        if item % self.0 == 0 {
            self.1
        } else {
            self.2
        }
    }
}

#[derive(Debug)]
struct Monkey(usize, u64, Vec<i64>, Operation, Test);

fn build_monkeys(monkey_desc: &str) -> Vec<RefCell<Monkey>> {
    monkey_desc.lines()
        .fold(
            (vec![], (None, None, None, None, None)),
            |(mut v, builder), l| {
                let l_spl = Vec::from_iter(l.split_whitespace());
                match (l_spl.as_slice(), builder) {
                    ([], (None, None, None, None, None)) => {
                        (v, (None, None, None, None, None))
                    },
                    (["Monkey", id], (None, None, None, None, None)) => {
                        let p_id = id.trim_end_matches(":").parse::<usize>().unwrap();
                        (v, (Some(p_id), None, None, None, None))
                    },
                    (["Starting", "items:", worries @ ..], (Some(m_id), None, None, None, None)) => {
                        let worry_list: Vec<i64> = worries.iter().map(|w| w.trim_end_matches(",").parse().unwrap()).collect();
                        (v, (Some(m_id), Some(worry_list), None, None, None))
                    },
                    (["Operation:", "new", "=", "old", "+", i], (Some(m_id), Some(wr_v), None, None, None)) => {
                        (v, (Some(m_id), Some(wr_v), Some(Operation::AddI(i.parse().unwrap())), None, None))
                    },
                    (["Operation:", "new", "=", "old", "*", "old"], (Some(m_id), Some(wr_v), None, None, None)) => {
                        (v, (Some(m_id), Some(wr_v), Some(Operation::Pow2), None, None))
                    },
                    (["Operation:", "new", "=", "old", "*", i], (Some(m_id), Some(wr_v), None, None, None)) => {
                        (v, (Some(m_id), Some(wr_v), Some(Operation::MultI(i.parse().unwrap())), None, None))
                    },
                    (["Test:", "divisible", "by", i], (Some(m_id), Some(wr_v), Some(op), None, None)) => {
                        (v, (Some(m_id), Some(wr_v), Some(op), Some(i.parse::<i64>().unwrap()), None))
                    },
                    (["If", "true:", "throw", "to", "monkey", i], (Some(m_id), Some(wr_v), Some(op), Some(div), None)) => {
                        (v, (Some(m_id), Some(wr_v), Some(op), Some(div), Some(i.parse::<usize>().unwrap())))
                    },
                    (["If", "false:", "throw", "to", "monkey", i], (Some(m_id), Some(wr_v), Some(op), Some(div), Some(t))) => {
                        v.push(RefCell::new(Monkey(m_id, 0, wr_v, op, Test(div, t, i.parse::<usize>().unwrap()))));
                        (v, (None, None, None, None, None))
                    },
                    _ => panic!("Unknown building step '{}'", l)
                }
            }
        ).0
}

fn monkey_bussiness(monkey_desc: &str, round_limit: usize, relief: bool) -> u64 {
    let monkeys = build_monkeys(monkey_desc);
    let cm: i64 = monkeys.iter().map(|m| m.borrow().4.0).product();
    for _ in 0..round_limit {
        for m_id in 0..(monkeys.len()) {
            let mut mon = monkeys.get(m_id).unwrap().borrow_mut();
            let drain: Vec<i64> = mon.2.drain(..).collect();
            for item in drain {
                mon.1 += 1;
                let new_w = if relief {
                    mon.3.exec(item) / 3
                } else {
                    mon.3.exec(item)
                };

                let receiver = mon.4.throw(new_w);
                monkeys.get(receiver).unwrap().borrow_mut().2.push(new_w % cm);
            }
        }
    }
    std::collections::BinaryHeap::from_iter(
        monkeys.iter()
            .map(
                |m| m.borrow().1
            )
        ).iter().take(2).product()
}

pub fn chase_monkeys(input_path: &str, round_limit: usize, relief: bool) -> u64 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => monkey_bussiness(&content, round_limit, relief),
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
r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn test_input1() {
        assert_eq!(monkey_bussiness(TEST_INP1, 20, true), 10605)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(monkey_bussiness(TEST_INP1, 10_000, false), 2713310158)
    }
}