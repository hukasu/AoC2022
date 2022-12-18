use std::collections::{HashSet, VecDeque};

fn get_cubes(cubes: &str) -> HashSet<(usize, usize, usize)> {
    cubes.lines()
        .map(
            |cube| {
                let coords: Vec<usize> = cube.split_terminator(",")
                    .map(|c| c.parse().unwrap())
                    .collect();
                if let &[x, y, z] = coords.as_slice() {
                    (x, y, z)
                } else {
                    panic!("Failed to parse cube '{:?}'", cube)
                }
            }
        )
        .collect()
}

fn get_total_surface(cubes: &HashSet<(usize, usize, usize)>, occupied: &mut HashSet<(usize, usize, usize)>) -> u32 {
    cubes.iter()
        .fold(
            0,
            |surface, (x, y, z)| {
                let mut sides = 6;
                if occupied.contains(&(x + 1, *y, *z)) {
                    sides -= 1;
                }
                if *x > 0 && occupied.contains(&(x - 1, *y, *z)) {
                    sides -= 1;
                }
                if occupied.contains(&(*x, y + 1, *z)) {
                    sides -= 1;
                }
                if *y > 0 && occupied.contains(&(*x, y - 1, *z)) {
                    sides -= 1;
                }
                if occupied.contains(&(*x, *y, z + 1)) {
                    sides -= 1;
                }
                if *z > 0 && occupied.contains(&(*x, *y, z - 1)) {
                    sides -= 1;
                }
                occupied.insert((*x, *y , *z));
                let adj = 6 - sides;
                surface - adj + sides
            }
        )
}

fn get_pockets_surface(occupied: &HashSet<(usize, usize, usize)>) -> u32 {
    let max_x = occupied.iter().map(|a| a.0).max_by(|a, b| a.cmp(&b)).unwrap();
    let max_y = occupied.iter().map(|a| a.1).max_by(|a, b| a.cmp(&b)).unwrap();
    let max_z = occupied.iter().map(|a| a.2).max_by(|a, b| a.cmp(&b)).unwrap();
    
    let mut air = HashSet::new();
    let mut queue = VecDeque::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            for z in 0..=max_z {
                if x == 0 || x == max_x || y == 0 || y == max_y || z == 0 || z == max_z {
                    queue.push_back((x, y, z));
                }
                if !occupied.contains(&(x, y, z)) {
                    air.insert((x, y, z));
                }
            }
        }
    }

    while !queue.is_empty() {
        let (x, y, z) = queue.pop_front().unwrap();
        if air.contains(&(x, y, z)) {
            air.remove(&(x, y, z));
            if x > 0 {
                queue.push_back((x - 1, y, z))
            }
            if x < max_x {
                queue.push_back((x + 1, y, z))
            }
            if y > 0 {
                queue.push_back((x, y - 1, z))
            }
            if y < max_y {
                queue.push_back((x, y + 1, z))
            }
            if z > 0 {
                queue.push_back((x, y, z - 1))
            }
            if z < max_z {
                queue.push_back((x, y, z + 1))
            }
        }
    }
    
    let mut occupied = HashSet::new();
    get_total_surface(&air, &mut occupied)
}

fn falling_rocks(cubes: &str, remove_pockets: bool) -> u32 {
    let mut occupied = HashSet::new();
    let cubes = get_cubes(cubes);
    let total_surface = get_total_surface(&cubes, &mut occupied);
    if remove_pockets {
        total_surface - get_pockets_surface(&occupied)
    } else {
        total_surface
    }
}

pub fn falling_rocks_surface(input_path: &str, remove_pockets: bool) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => falling_rocks(&content, remove_pockets),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

// pub fn release_pressure_with_help(input_path: &str) -> u32 {
//     let content = std::fs::read_to_string(input_path);
//     match content {
//         Ok(content) => depressurize_with_help(&content),
//         Err(er) => {
//             println!("{}", er);
//             0
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INP1: &str = 
r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    #[test]
    fn test_input1() {
        assert_eq!(falling_rocks(TEST_INP1, false), 64)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(falling_rocks(TEST_INP1, true), 58)
    }
}