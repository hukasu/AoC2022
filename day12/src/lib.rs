fn build_height_map(height_map_code: &str) -> ((usize, usize), (usize, usize), Vec<Vec<u32>>) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let heights = height_map_code.lines().enumerate()
        .map(
            |(i, l)| {
                l.chars().enumerate()
                    .map(
                        |(j, c)| {
                            if c == 'S' {
                                start = (i, j);
                                0
                            } else if c == 'E' {
                                end = (i, j);
                                ('z' as u32) - ('a' as u32)
                            } else {
                                (c as u32) - ('a' as u32)
                            }
                        }
                    )
                    .collect()
            }
        )
        .collect();
    (start, end, heights)
}

#[allow(dead_code)]
fn print_2d_matrix(matrix: &Vec<Vec<u32>>) {
    matrix.iter()
        .for_each(
            |v| {
                v.iter().for_each(|c| print!("{c:03} "));
                println!("");
            }
        );
}

fn advance_trek(
    height_map: &Vec<Vec<u32>>,
    steps: &mut Vec<Vec<u32>>,
    queue: &mut std::collections::VecDeque<(usize, usize)>,
    dups: &mut std::collections::BTreeSet<(usize, usize)>,
    end: (usize, usize)
) -> u32 {
    loop {
        if let Some((x, y)) = queue.pop_front() {
            if end == (x, y) {
                break steps[x][y];
            }
            dups.remove(&(x, y));
            let mut adj = vec![];
            let cur_height = height_map[x][y];
            let cur_steps = steps[x][y];
            if x > 0 {
                adj.push((x - 1, y));
            }
            if x < height_map.len() - 1 {
                adj.push((x + 1, y));
            }
            if y > 0 {
                adj.push((x, y - 1));
            }
            if y < height_map[0].len() - 1 {
                adj.push((x, y + 1));
            }
            adj.into_iter()
                .for_each(
                    |(x, y)| {
                        if steps[x][y] > cur_steps + 1 && height_map[x][y] as i32 - cur_height as i32 <= 1 {
                            if !dups.contains(&(x, y)) {
                                queue.push_back((x, y));
                                dups.insert((x, y));
                                steps[x][y] = cur_steps + 1;
                            }
                        }
                    }
                );
        } else {
            break 0;
        }
    }
}

fn trek(height_map_code: &str) -> u32 {
    let (start, end, height_map) = build_height_map(height_map_code);
    let mut steps = vec![vec![999; height_map[0].len()]; height_map.len()];
    steps[start.0][start.1] = 0;
    let mut queue = std::collections::VecDeque::from_iter([start]);
    let mut dups = std::collections::BTreeSet::from_iter([start]);
    advance_trek(&height_map, &mut steps, &mut queue, &mut dups, end)
}

pub fn climb_to_best_reception(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => trek(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn scenic_trek(height_map_code: &str) -> u32 {
    let (start, end, height_map) = build_height_map(height_map_code);
    let mut steps = vec![vec![999; height_map[0].len()]; height_map.len()];
    steps[start.0][start.1] = 0;
    let mut queue = std::collections::VecDeque::new();
    let mut dups = std::collections::BTreeSet::new();
    
    height_map.iter().enumerate()
        .for_each(
            |(i, v)| {
                v.iter().enumerate()
                    .for_each(
                        |(j, height)| {
                            if height == &0 {
                                queue.push_back((i, j));
                                dups.insert((i, j));
                                steps[i][j] = 0;
                            }
                        }
                    )
            }
        );
    
    advance_trek(&height_map, &mut steps, &mut queue, &mut dups, end)
}

pub fn find_scenic_trek(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => scenic_trek(&content),
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
r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn test_input1() {
        assert_eq!(trek(TEST_INP1), 31)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(scenic_trek(TEST_INP1), 29)
    }
}