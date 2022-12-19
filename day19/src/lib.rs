use std::collections::{VecDeque, BTreeSet};

#[derive(Debug)]
struct Blueprint {
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32)
}

fn parse_blueprints(blueprints: &str) -> Vec<Blueprint> {
    blueprints.lines()
        .map(
            |l| {
                let spl = l.split_whitespace();
                let vspl = Vec::from_iter(spl);
                if let [
                    "Blueprint", _id,
                    "Each", "ore", "robot", "costs", ore, "ore.",
                    "Each", "clay", "robot", "costs", clay, "ore.",
                    "Each", "obsidian", "robot", "costs", obs_ore, "ore", "and", obs_clay, "clay.",
                    "Each", "geode", "robot", "costs", geo_ore, "ore", "and", geo_obs, "obsidian."
                ] = vspl.as_slice() {
                    Blueprint {
                        ore: ore.parse().unwrap(),
                        clay: clay.parse().unwrap(),
                        obsidian: (obs_ore.parse().unwrap(), obs_clay.parse().unwrap()),
                        geode: (geo_ore.parse().unwrap(), geo_obs.parse().unwrap())
                    }
                } else {
                    panic!("Failed to parse blueprint '{}'", l)
                }
            }
        )
        .collect()
}

fn time_to_robot(blueprint: &Blueprint, robots: &[u32; 4], storage: &[u32; 4]) -> [u32; 4] {
    let req_for_geode = (
        blueprint.geode.0.max(storage[0]) - storage[0],
        blueprint.geode.1.max(storage[2]) - storage[2]
    );
    let time_to_geode = if robots[2] > 0 {
        (
            (req_for_geode.0 / robots[0]) + if req_for_geode.0 % robots[0] == 0 {
                0
            } else {
                1
            }
        ).max(
            req_for_geode.1 / robots[2] + if req_for_geode.1 % robots[2] == 0 {
                0
            } else {
                1
            }
        )
    } else {
        std::u16::MAX as u32
    };

    let req_for_obsidian = (
        blueprint.obsidian.0.max(storage[0]) - storage[0],
        blueprint.obsidian.1.max(storage[1]) - storage[1]
    );
    let time_to_obsidian = if robots[1] > 0 {
        (
            (req_for_obsidian.0 / robots[0]) + if req_for_obsidian.0 % robots[0] == 0 {
                0
            } else {
                1
            }
        ).max(
            (req_for_obsidian.1 / robots[1]) + if req_for_obsidian.1 % robots[1] == 0 {
                0
            } else {
                1
            }
        )
    }else {
        std::u16::MAX as u32
    };

    let req_for_clay = blueprint.clay.max(storage[0]) - storage[0];
    let time_to_clay = (req_for_clay / robots[0]) + if req_for_clay % robots[0] == 0 {
        0
    } else {
        1
    };
    

    let req_for_ore = blueprint.ore.max(storage[0]) - storage[0];
    let time_to_ore = (req_for_ore / robots[0]) + if req_for_ore % robots[0] == 0 {
        0
    } else {
        1
    };

    [time_to_ore, time_to_clay, time_to_obsidian, time_to_geode]
}

fn geode_cracking(blueprint: &Blueprint, time_limit: u32) -> u32 {
    let mut queue = VecDeque::new();
    (0..4).for_each(
        |i| {
            queue.push_back((i, time_limit, [1_u32, 0, 0, 0], [0_u32; 4]));
        }
    );

    let mut ended = BTreeSet::new();
    while !queue.is_empty() {
        let (target, time_left, robots, storage) = queue.pop_back().unwrap();
        // Timestep can't be lower than 1
        let timestep = time_to_robot(blueprint, &robots, &storage)[target] + 1;
        if time_left <= timestep {
            ended.insert(storage[3] + (robots[3] * time_left));
        } else {
            let mut n_storage = storage.clone();
            (0..4).for_each(
                |i| n_storage[i] += robots[i] * timestep
            );
            
            match target {
                0 => n_storage[0] -= blueprint.ore,
                1 => n_storage[0] -= blueprint.clay,
                2 => {
                    n_storage[0] -= blueprint.obsidian.0;
                    n_storage[1] -= blueprint.obsidian.1;
                },
                3 => {
                    n_storage[0] -= blueprint.geode.0;
                    n_storage[2] -= blueprint.geode.1;
                },
                _ => panic!("Unknown robot")
            }

            let mut n_robots = robots.clone();
            n_robots[target] += 1;

            (0..4).for_each(
                |i| {
                    // Hint seen on reddit by u/Boojum
                    // Don't build robot if already producing enough resource per second for most expensive recipe
                    let dont_branch = match i {
                        0 => {
                            [blueprint.ore, blueprint.clay, blueprint.obsidian.0, blueprint.geode.0].into_iter()
                            .max()
                            .unwrap() <= n_robots[0]
                        },
                        1 => blueprint.obsidian.1 <= n_robots[1],
                        2 => blueprint.geode.1 <= n_robots[2],
                        3 => false,
                        _ => panic!("Unknown robot")
                    };
                    if dont_branch {
                        ended.insert(n_storage[3] + (n_robots[3] * (time_left - timestep)));
                    } else {
                        let n = (i, time_left - timestep, n_robots, n_storage);
                        queue.push_back(n);
                        // if !queue.contains(&n) {
                            
                        // }
                    }
                }
            );
        }
    }
    ended.into_iter()
        .max()
        .unwrap()
}

fn idler_game(blueprints: &str) -> u32 {
    let blueprints = parse_blueprints(blueprints);
    blueprints.iter().enumerate()
        .map(
            |(i, bp)| {
                (1 + i as u32) * geode_cracking(bp, 24)
            }
        )
        .sum()
}

pub fn run_idler_game(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => idler_game(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn idler_game_hungry_elephants(blueprints: &str) -> u32 {
    let blueprints = parse_blueprints(blueprints);
    blueprints.iter()
        .take(3)
        .map(
            |bp| {
                geode_cracking(bp, 32)
            }
        )
        .product()
}

pub fn run_idler_game_hungry_elephants(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => idler_game_hungry_elephants(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_to_robot_test() {
        let bp = Blueprint{
            ore: 4,
            clay: 2,
            obsidian: (3, 14),
            geode: (2, 7)
        };
        
        let robots = [1, 0, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 4);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 2);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 0, 0, 0];
        let storage = [1, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 3);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 0, 0, 0];
        let storage = [2, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 2);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 0);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 0, 0, 0];
        let storage = [3, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 0);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 0, 0, 0];
        let storage = [4, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 0);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 0);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 0, 0, 0];
        let storage = [5, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 0);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 0);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [2, 0, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 2);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [3, 0, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 2);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [4, 0, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [5, 0, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [8, 0, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 1, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 14);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 2, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 7);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 3, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 5);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 4, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 4);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 5, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 3);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 6, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 3);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [1, 7, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 3);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [2, 7, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 2);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [2, 13, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 2);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [2, 14, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 2);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [3, 14, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);
        
        let robots = [3, 15, 0, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], std::u32::MAX);

        let robots = [1, 0, 1, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], 7);

        let robots = [7, 0, 1, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], 7);

        let robots = [1, 0, 7, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], 2);

        let robots = [2, 0, 7, 0];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], std::u32::MAX);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], 1);
        
        let robots = [100, 100, 100, 100];
        let storage = [0, 0, 0, 0];
        assert_eq!(time_to_robot(&bp, &robots, &storage)[0], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[1], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[2], 1);
        assert_eq!(time_to_robot(&bp, &robots, &storage)[3], 1);
    }

    const TEST_INP1: &str = 
r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."#;

    #[test]
    fn test_input1() {
        assert_eq!(idler_game(TEST_INP1), 33)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(idler_game_hungry_elephants(TEST_INP1), 3596)
    }
}