enum ShadowDirection {
    North,
    South,
    East,
    West
}

fn is_shadowed_bidirectional(a: &Vec<Vec<bool>>, b: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    a.iter()
        .zip(b)
        .map(
            |(n, s)| {
                n.iter().zip(s)
                    .map(|(n, s)| *n || *s)
                    .collect()
            }
        )
        .collect()
}

fn build_shadow_map(tree_matrix: &Vec<Vec<u32>>, direction: ShadowDirection) -> Vec<Vec<bool>> {
    let mut shadow_height = tree_matrix.clone();
    let mut res = vec![vec![true; tree_matrix[0].len()]; tree_matrix.len()];
    
    match direction {
        ShadowDirection::North => {
            for i in (0..(tree_matrix.len() - 1)).rev() {
                for j in 0..(tree_matrix[0].len()) {
                    res[i][j] = shadow_height[i][j].cmp(&shadow_height[i + 1][j]) == std::cmp::Ordering::Greater;
                    shadow_height[i][j] = (shadow_height[i + 1][j]).max(shadow_height[i][j]);
                }
            }
        },
        ShadowDirection::South => {
            for i in 1..(tree_matrix.len()) {
                for j in 0..(tree_matrix[0].len()) {
                    res[i][j] = shadow_height[i][j].cmp(&shadow_height[i - 1][j]) == std::cmp::Ordering::Greater;
                    shadow_height[i][j] = (shadow_height[i - 1][j]).max(shadow_height[i][j]);
                }
            }
        },
        ShadowDirection::East => {
            for i in 0..(tree_matrix.len()) {
                for j in 1..(tree_matrix[0].len()) {
                    res[i][j] = shadow_height[i][j].cmp(&shadow_height[i][j - 1]) == std::cmp::Ordering::Greater;
                    shadow_height[i][j] = (shadow_height[i][j - 1]).max(shadow_height[i][j]);
                }
            }
        },
        ShadowDirection::West => {
            for i in 0..(tree_matrix.len()) {
                for j in (0..(tree_matrix[0].len() - 1)).rev() {
                    res[i][j] = shadow_height[i][j].cmp(&shadow_height[i][j + 1]) == std::cmp::Ordering::Greater;
                    shadow_height[i][j] = (shadow_height[i][j + 1]).max(shadow_height[i][j]);
                }
            }
        },
    }

    res
}

fn build_tree_height_matrix(tree_heights: &str) -> Vec<Vec<u32>> {
    tree_heights.lines()
        .map(
            |l| l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        )
        .collect()
}

fn _count_visible_trees(tree_heights: &str) -> u32 {
    let tree_matrix = build_tree_height_matrix(tree_heights);
    // Vectors represent which trees are visible when looking at a certain direction
    let north_shadow = build_shadow_map(&tree_matrix, ShadowDirection::North);
    let south_shadow = build_shadow_map(&tree_matrix, ShadowDirection::South);
    let east_shadow = build_shadow_map(&tree_matrix, ShadowDirection::East);
    let west_shadow = build_shadow_map(&tree_matrix, ShadowDirection::West);

    let visible_north_south = is_shadowed_bidirectional(&north_shadow, &south_shadow);
    let visible_east_west = is_shadowed_bidirectional(&east_shadow, &west_shadow);

    let visible = is_shadowed_bidirectional(&visible_north_south, &visible_east_west);
    
    let shadowed = visible.iter()
        .flatten()
        .filter(|b| **b)
        .count();
    (shadowed) as u32
}

pub fn count_visible_trees(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => _count_visible_trees(&content),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn count_view_north(tree_matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for tx in (0..x).rev() {
        count = count + 1;
        if tree_matrix[x][y] <= tree_matrix[tx][y] {
            break;
        }
    }
    count
}

fn count_view_south(tree_matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for tx in (x + 1)..(tree_matrix.len()) {
        count = count + 1;
        if tree_matrix[x][y] <= tree_matrix[tx][y] {
            break;
        }
    }
    count
}

fn count_view_east(tree_matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for ty in (y + 1)..(tree_matrix[0].len()) {
        count = count + 1;
        if tree_matrix[x][y] <= tree_matrix[x][ty] {
            break;
        }
    }
    count
}

fn count_view_west(tree_matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for ty in (0..y).rev() {
        count = count + 1;
        if tree_matrix[x][y] <= tree_matrix[x][ty] {
            break;
        }
    }
    count
}

fn tree_scenic_score(tree_matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    count_view_north(tree_matrix, x, y) *
    count_view_south(tree_matrix, x, y) *
    count_view_east(tree_matrix, x, y) *
    count_view_west(tree_matrix, x, y)
}

fn _find_best_scenic_score(tree_heights: &str) -> u32 {
    let tree_matrix = build_tree_height_matrix(tree_heights);
    tree_matrix.iter().enumerate()
        .map(
            |(x, v)| {
                v.iter().enumerate()
                    .map(
                        |(y, _)| {
                            tree_scenic_score(&tree_matrix, x, y)
                        }
                    )
                    .max()
                    .unwrap_or(0)
            }
        )
        .max()
        .unwrap_or(0)
}

pub fn find_best_scenic_score(input_path: &str) -> u32 {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => _find_best_scenic_score(&content),
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
r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_input1() {
        assert_eq!(_count_visible_trees(TEST_INP1), 21)
    }

    #[test]
    fn test_input1_part2_test1() {
        let tree_matrix = build_tree_height_matrix(TEST_INP1);
        assert_eq!(tree_scenic_score(&tree_matrix, 1, 2), 4)
    }

    #[test]
    fn test_input1_part2_test2() {
        let tree_matrix = build_tree_height_matrix(TEST_INP1);
        assert_eq!(tree_scenic_score(&tree_matrix, 3, 2), 8)
    }
}