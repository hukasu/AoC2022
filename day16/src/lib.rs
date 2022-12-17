use std::collections::{HashMap, VecDeque, HashSet};

const MAX_TRAVEL: u32 = std::u16::MAX as u32;

fn build_valve_layout(cave: &str) -> (HashMap<String, u32>, HashMap<String, Vec<String>>) {
    cave.replace(';', "")
        .replace(',', "")
        .replace('=', " ")
        .lines()
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut valves, mut tunnels), l| {
                let spl = l.split_terminator(' ');
                let v = Vec::from_iter(spl);
                match v.as_slice() {
                    [
                        "Valve",
                        val,
                        "has",
                        "flow",
                        "rate",
                        fr,
                        "tunnel" | "tunnels",
                        "leads" | "lead",
                        "to",
                        "valve" | "valves",
                        a @ ..
                    ] => {
                        valves.insert(String::from(*val), fr.parse().unwrap());
                        tunnels.insert(String::from(*val), a.iter().map(|t| String::from(*t)).collect());
                    },
                    _ => panic!("Malformed valve report '{l}'")
                }
                (valves, tunnels)
            }
        )
}

fn build_travel_matrix(tunnels: &HashMap<String, Vec<String>>) -> HashMap<(String, String), u32> {
    tunnels.iter()
        .fold(
            HashMap::new(),
            |mut adj, (cur, _)| {
                adj.insert((cur.clone(), cur.clone()), 0);
                let mut queue = VecDeque::from_iter([cur]);
                while !queue.is_empty() {
                    let dest = queue.pop_front().unwrap();
                    let conn_to_dest = tunnels.get(dest).unwrap();
                    for cd in conn_to_dest {
                        let cur_to_dest = *adj.get(&(cur.clone(), dest.clone())).unwrap_or(&MAX_TRAVEL);
                        let cur_to_conn = *adj.get(&(cur.clone(), cd.clone())).unwrap_or(&MAX_TRAVEL);
                        if cur_to_dest + 1 < cur_to_conn {
                            adj.insert((cur.clone(), cd.clone()), cur_to_dest + 1);
                            queue.push_back(cd);
                        }
                    }
                }
                adj
            }
        )
}

fn max_expected_depressure(
    position: &String,
    next: String,
    time_left: u32,
    valves: &HashMap<String, u32>,
    travel: &HashMap<(String, String), u32>
) -> (u32, u32) {
    let expected_travel = *travel.get(&(position.clone(), next.clone())).unwrap();
    let flow = valves.get(&next).unwrap();
    if *flow == 0 {
        ((expected_travel + 1), 0)
    } else if expected_travel >= time_left {
        ((expected_travel + 1), 0)
    } else {
        ((expected_travel + 1), (time_left - (expected_travel + 1)) * flow)
    }
}

fn fan_out(
    cur: &String,
    cur_partner: &String,
    time_left: u32,
    valves: &HashMap<String, u32>,
    travel_time: &HashMap<(String, String), u32>,
    opened: &HashSet<String>
) -> Vec<(String, u32, u32)> {
    valves.iter()
        .filter_map(
            |(dest, _)| {
                if !opened.contains(dest) && dest != cur && dest != cur_partner {
                    let (time_taken, expected) = max_expected_depressure(
                        &cur,
                        dest.clone(),
                        time_left,
                        &valves,
                        &travel_time
                    );
                    if time_left >= time_taken && expected > 0 {
                        Some((dest.clone(), time_taken, expected))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        )
        .collect()
}

fn depressurize(cave: &str) -> u32 {
    let (valves, tunnels) = build_valve_layout(cave);
    let travel_time = build_travel_matrix(&tunnels);
    let mut queue = VecDeque::new();
    queue.push_back((String::from("AA"), 30, 0, vec![String::from("AA")]));
    let mut ended = vec![];
    while !queue.is_empty() {
        let (cur, time_left, released, opened) = queue.pop_front().unwrap();
        println!("Currently at {} with {} minutes left, expected output of {}, opened {:?}", cur, time_left, released, opened);
        let fanned = fan_out(
            &cur,
            &String::from("AA"),
            time_left,
            &valves,
            &travel_time,
            &HashSet::from_iter(opened.iter().map(|s| s.clone()))
        );
        if fanned.is_empty() {
            ended.push(released)
        } else {
            fanned.iter()
                .for_each(
                    |(dest, time_taken, expected)| {
                        let mut op_cl = opened.clone();
                        op_cl.append(&mut vec![dest.clone()]);
                        queue.push_back(
                            (
                                dest.clone(),
                                time_left - time_taken,
                                released + expected,
                                op_cl
                            )
                        )
                    }
                )
        }
    }
    ended.into_iter()
        .max()
        .unwrap()
}

pub fn release_pressure(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => depressurize(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn depressurize_with_help(cave: &str) -> u32 {
    let (valves, tunnels) = build_valve_layout(cave);
    let travel_time = build_travel_matrix(&tunnels);
    let mut queue = VecDeque::new();
    queue.push_back(
        (
            (String::from("AA"), 0, String::from("AA")),
            (String::from("AA"), 0, String::from("AA")),
            26,
            0,
            vec![String::from("AA")]
        )
    );
    let mut ended = vec![];
    while !queue.is_empty() {
        let (
            cur,
            eleph,
            time_left,
            released,
            opened
        ) = queue.pop_front().unwrap();
        // println!("===");
        // println!("You are at {}, moving towards {} ariving in {} minutes", cur.0, cur.2, cur.1);
        // println!("Elephant at {}, moving towards {} ariving in {} minutes", eleph.0, eleph.2, eleph.1);
        // println!("{} minutes left, expected output of {}, opened {:?}",time_left, released, opened);
        if time_left == 0 {
            ended.push(released)
        } else if cur.1 == 0 {
            let fanned = fan_out(
                &cur.2,
                &eleph.2,
                time_left,
                &valves,
                &travel_time,
                &HashSet::from_iter(opened.iter().map(|s| s.clone()))
            );
            if fanned.is_empty() {
                ended.push(released)
            } else {
                fanned.iter()
                    .for_each(
                        |(dest, time_taken, expected)| {
                            let mut op_cl = opened.clone();
                            op_cl.push(cur.2.clone());
                            queue.push_back(
                                (
                                    (cur.2.clone(), *time_taken, dest.clone()),
                                    eleph.clone(),
                                    time_left,
                                    released + expected,
                                    op_cl
                                )
                            )
                        }
                    )
            }
        } else if eleph.1 == 0 {
            let fanned = fan_out(
                &eleph.2,
                &cur.2,
                time_left,
                &valves,
                &travel_time,
                &HashSet::from_iter(opened.iter().map(|s| s.clone()))
            );
            if fanned.is_empty() {
                ended.push(released)
            } else {
                fanned.iter()
                    .for_each(
                        |(dest, time_taken, expected)| {
                            let mut op_cl = opened.clone();
                            op_cl.push(eleph.2.clone());
                            queue.push_back(
                                (
                                    cur.clone(),
                                    (eleph.2.clone(), *time_taken, dest.clone()),
                                    time_left,
                                    released + expected,
                                    op_cl
                                )
                            )
                        }
                    )
            }
        } else {
            let timestep = cur.1.min(eleph.1);
            queue.push_back(
                (
                    (cur.0.clone(), cur.1 - timestep, cur.2.clone()),
                    (eleph.0.clone(), eleph.1 - timestep, eleph.2.clone()),
                    time_left - timestep,
                    released,
                    opened
                )
            )
        }
    }
    ended.into_iter()
        .max()
        .unwrap()
}

pub fn release_pressure_with_help(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => depressurize_with_help(&content),
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
r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    fn test_input1() {
        assert_eq!(depressurize(TEST_INP1), 1651)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(depressurize_with_help(TEST_INP1), 1707)
    }
}