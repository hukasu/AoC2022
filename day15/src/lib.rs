use std::collections::{BTreeMap, HashSet};

const BOUND: isize = 4_000_000;

fn manhatan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn build_sensor_response(cave: &str) -> BTreeMap<(isize, isize), (isize, isize)> {
    cave.lines()
        .map(
            |l| {
                let bounds: &[_] = &[' ', ':', ',', '='];
                let spl = l.split_terminator(bounds);
                let v = Vec::from_iter(spl);
                match v.as_slice() {
                    ["Sensor", "at", "x", sx, "", "y", sy, "", "closest", "beacon", "is", "at", "x", bx, "", "y", by] => {
                        (
                            (sx.parse().unwrap(), sy.parse().unwrap()),
                            (bx.parse().unwrap(), by.parse().unwrap())
                        )
                    },
                    _ => panic!("Malformed sensor response '{}'", l)
                }
            }
        )
        .fold(
            BTreeMap::new(),
            |mut hm, (s, b)| {
                hm.insert(s, b);
                hm
            }
        )
}

fn in_range(sensors: &BTreeMap<(isize, isize), (isize, isize)>, pos: (isize, isize)) -> bool {
    sensors.iter()
        .map(
            |(s, b)| {
                if pos == *s || pos == *b {
                    false
                } else if manhatan_distance(*s, *b) >= manhatan_distance(*s, pos) {
                    true
                } else {
                    false
                }
            }
        )
        .any(|b| b)       
}

fn area_bounds(sensors: &BTreeMap<(isize, isize), (isize, isize)>) -> ((isize, isize), (isize, isize)) {
    let objects = sensors.iter()
        .fold(
            HashSet::new(),
            |mut hs, (s, b)| {
                let dist = manhatan_distance(*s, *b) as isize;
                hs.insert((s.0 - dist, s.1));
                hs.insert((s.0 + dist, s.1));
                hs.insert((s.0, s.1 - dist));
                hs.insert((s.0, s.1 + dist));
                hs
            }
        );
    let min_x = objects.iter()
        .min_by(
            |a, b| a.0.cmp(&b.0)
        ).unwrap().0;
    let max_x = objects.iter()
        .max_by(
            |a, b| a.0.cmp(&b.0)
        ).unwrap().0;
    let min_y = objects.iter()
        .min_by(
            |a, b| a.1.cmp(&b.1)
        ).unwrap().1;
    let max_y = objects.iter()
        .max_by(
            |a, b| a.1.cmp(&b.1)
        ).unwrap().1;
    ((min_x, max_x), (min_y, max_y))
}

#[allow(dead_code)]
fn print_ranges(sensors: &BTreeMap<(isize, isize), (isize, isize)>, depth: isize) {
    let ((min_x, max_x), (min_y, max_y)) = area_bounds(sensors);
    
    let beacons = sensors.iter()
        .fold(
            HashSet::new(),
            |mut hs, (_, b)| {
                hs.insert(*b);
                hs
            }
        );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if sensors.contains_key(&(x, y)) {
                print!("S")
            } else if beacons.contains(&(x, y)) {
                print!("B")
            } else if in_range(sensors, (x, y)) {
                print!("#")
            } else if y == depth {
                print!("~")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

fn probe(cave: &str, depth: isize) -> u32 {
    let sensors = build_sensor_response(cave);
    let ((min_x, max_x), (_, _)) = area_bounds(&sensors);
    ((min_x)..=max_x).into_iter()
        .filter_map(
            |x| {
                if in_range(&sensors, (x, depth)) {
                    Some(x)
                } else {
                    None
                }
            }
        )
        .count() as u32
}

pub fn probe_depth(input_path: &str, depth: isize) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => probe(&content, depth),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn tune_frequency(cave: &str) -> u128 {
    let sensors = build_sensor_response(cave);
    (0..=BOUND).into_iter()
        .filter_map(
            |y| {
                let range = sensors.iter()
                    .fold(
                        std::ops::RangeInclusive::new(0, 0),
                        |r, (s, b)| {
                            let bounds = r.into_inner();
                            let radius = manhatan_distance(*s, *b);
                            if radius < manhatan_distance(*s, (s.0, y)) {
                                std::ops::RangeInclusive::new(bounds.0, bounds.1)
                            } else {
                                let vert_dist = s.1 - y;
                                let hor_dist = radius as isize - vert_dist.abs();
                                let range = (s.0 - hor_dist, s.0 + hor_dist);
                                // o---o
                                //        o---o
                                if (bounds.1 + 1) < range.0 {
                                    std::ops::RangeInclusive::new(bounds.0, bounds.1)
                                // o---o
                                //     o---o
                                } else if bounds.0 < range.0 && (bounds.1 + 1) >= range.0 {
                                    std::ops::RangeInclusive::new(bounds.0, bounds.1.max(range.1))
                                //  o---o
                                // o-------o
                                } else if bounds.0 >= range.0 {
                                    std::ops::RangeInclusive::new(bounds.0.min(range.0), bounds.1.max(range.1))
                                } else {
                                    std::ops::RangeInclusive::new(bounds.0, bounds.1)
                                }
                            }
                        }
                    )
                    .into_inner();
                match (range.0.cmp(&0), range.1.cmp(&BOUND)) {
                    (std::cmp::Ordering::Less | std::cmp::Ordering::Equal, std::cmp::Ordering::Equal | std::cmp::Ordering::Greater) => {
                        None
                    },
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal | std::cmp::Ordering::Greater) => {
                        Some((range.0 - 1) as u128 * 4_000_000 + y as u128)
                    },
                    (std::cmp::Ordering::Less | std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                        Some((range.1 + 1) as u128 * 4_000_000 + y as u128)
                    },
                    _ => panic!("Depth not fully covered ({:?})", range)
                }
            }
        )
        .max()
        .unwrap()
}

pub fn find_beacon_frequency(input_path: &str) -> u128 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => tune_frequency(&content),
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
r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn test_input1() {
        assert_eq!(probe(TEST_INP1, 10), 26)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(tune_frequency(TEST_INP1), 56000011)
    }
}