use std::fmt::Display;

enum FSNode {
    Directory(String, usize, Vec<usize>),
    File(String, usize)
}

impl Display for FSNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FSNode::Directory(name, _, _) => write!(f, "{}", name),
            FSNode::File(name, size) => write!(f, "{} {}", size, name)
        }
    }
}

impl FSNode {
    fn name_of_index(storage: &Vec<Self>, index: usize) -> &str {
        match storage.get(index).unwrap() {
            FSNode::Directory(name, _, _) => name,
            FSNode::File(name, _) => name
        }
    }

    fn parent_of_index(storage: &Vec<Self>, index: usize) -> usize {
        match storage.get(index).unwrap() {
            FSNode::Directory(_, par, _) => *par,
            _ => panic!("Current dir was file")
        }
    }

    fn index_has_child(storage: &Vec<Self>, current: usize, file: &str) -> Option<usize> {
        match storage.get(current).unwrap() {
            FSNode::Directory(_, _, children) => {
                for i in children {
                    if Self::name_of_index(storage, *i) == file.to_string() {
                        return Some(*i)
                    }
                };
                None
            }
            _ => None
        }
    }

    fn cd_from_index(storage: &mut Vec<Self>, current: usize, cd: &str) -> usize {
        let index = Self::index_has_child(storage, current, cd);
        let new_dir = FSNode::Directory(cd.to_string(), current, vec![]);
        index
            .unwrap_or(
                {
                    let l = storage.len();
                    storage.push(new_dir);
                    storage.get_mut(current).unwrap().add_node(l);
                    l
                }
            )
    }

    fn add_dir_to_index(storage: &mut Vec<Self>, current: usize, dir: &str) {
        let index = Self::index_has_child(storage, current, dir);
        let new_dir = FSNode::Directory(dir.to_string(), current, vec![]);
        match index {
            None => {
                let l = storage.len();
                storage.push(new_dir);
                storage.get_mut(current).unwrap().add_node(l);
            },
            _ => panic!("Directory already exists")
        }
    }

    fn add_file_to_index(storage: &mut Vec<Self>, current: usize, file: &str, size: usize) {
        let index = Self::index_has_child(storage, current, file);
        let new_file = FSNode::File(file.to_string(), size);
        match index {
            None => {
                let l = storage.len();
                storage.push(new_file);
                storage.get_mut(current).unwrap().add_node(l);
            },
            _ => panic!("Directory already exists")
        }
    }

    fn get_size_if_at_most(storage: &Vec<Self>, at_most: usize) -> usize {
        storage.iter().enumerate()
            .filter(|(_, fs)| match fs { FSNode::Directory(_, _, _) => true, _ => false})
            .map(|(i, _)| Self::get_size_of_index(storage, i))
            .filter(|s| *s <= at_most)
            .sum()
    }

    fn get_size_if_not_less(storage: &Vec<Self>, not_less: usize) -> usize {
        storage.iter().enumerate()
            .filter(|(_, fs)| match fs { FSNode::Directory(_, _, _) => true, _ => false})
            .map(|(i, _)| Self::get_size_of_index(storage, i))
            .filter(|s| *s >= not_less)
            .min().unwrap()
    }

    fn get_size_of_index(storage: &Vec<Self>, index: usize) -> usize {
        let cur = storage.get(index).unwrap();
        match cur {
            FSNode::Directory(_, _, children) => {
                children.iter()
                    .map(|fs| Self::get_size_of_index(storage, *fs))
                    .sum()
            }
            FSNode::File(_, size) => *size,
        }
    }

    fn display_index(storage: &Vec<Self>, index: usize, indent: usize) {
        for _ in 0..indent {
            print!("| ");
        }
        print!("|-");
        let fs = storage.get(index).unwrap();
        println!("{}", fs);
        match fs {
            FSNode::Directory(_, _, children) => {
                for c in children {
                    Self::display_index(storage, *c, indent + 1)
                }
            },
            _ => ()
        }
    }

    fn add_node(&mut self, node: usize) {
        match self {
            FSNode::Directory(_, _, children) => children.push(node),
            _ => panic!("Current dir was file")
        }
    }
}

const ROOT_PATH: &str = "/";

fn build_dic_tree_from_console(console: &str) -> Vec<FSNode> {
    let lines = console.split_terminator("\n");

    let mut dic_tree = vec![FSNode::Directory(ROOT_PATH.to_string(), 0, vec![])];
    let mut current_dir = 0;

    let mut reading_ls = false;
    for l in lines {
        let ln = Vec::from_iter(l.trim().split_terminator(" "));
        match (ln.as_slice(), reading_ls) {
            (["$", "cd", "/"], false) => current_dir = 0,
            (["$", "cd", "/"], true) => {
                current_dir = 0;
                reading_ls = false;
            },
            (["$", "cd", ".."], false) => {
                current_dir = FSNode::parent_of_index(&dic_tree, current_dir);
            },
            (["$", "cd", ".."], true) => {
                current_dir = FSNode::parent_of_index(&dic_tree, current_dir);
                reading_ls = false;
            },
            (["$", "cd", dir], false) => {
                current_dir = FSNode::cd_from_index(&mut dic_tree, current_dir, dir);
            },
            (["$", "cd", dir], true) => {
                current_dir = FSNode::cd_from_index(&mut dic_tree, current_dir, dir);
                reading_ls = false;
            },
            (["$", "ls"], false) => reading_ls = true,
            (["$", "ls"], true) => reading_ls = true,
            (["dir", dir], true) => {
                FSNode::add_dir_to_index(&mut dic_tree, current_dir, dir);
            },
            ([size, file], true) => {
                FSNode::add_file_to_index(&mut dic_tree, current_dir, file, size.parse().unwrap());
            },
            ln_err => panic!("Unresolved line '{:?}' ({})", ln_err, reading_ls)
        }
    }

    dic_tree
}

fn directories_with_at_most_recursive(console: &str, at_most: usize) -> usize {
    let dic_storage = build_dic_tree_from_console(console);
    FSNode::get_size_if_at_most(&dic_storage, at_most)
}

pub fn process_console(input_path: &str, at_most: usize) -> usize {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => directories_with_at_most_recursive(&content, at_most),
        Err(er) => {
            println!("{}", er);
            0
        }
    }
}

fn select_directory_until_available(console: &str, total_size: usize, until_available: usize) -> usize {
    let dic_storage = build_dic_tree_from_console(console);
    let current_available = total_size - FSNode::get_size_of_index(&dic_storage, 0);
    if until_available < current_available {
        0
    } else {
        FSNode::get_size_if_not_less(&dic_storage, until_available - current_available)
    }
}

pub fn select_directory_for_deletion(input_path: &str, total_size: usize, until_available: usize) -> usize {
    let content = std::fs::read_to_string(input_path);
    match content {
        Ok(content) => select_directory_until_available(&content, total_size, until_available),
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
r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_input1() {
        assert_eq!(directories_with_at_most_recursive(TEST_INP1, 100000), 95437)
    }

    #[test]
    fn test_input1_part2() {
        assert_eq!(select_directory_until_available(TEST_INP1, 70_000_000, 30_000_000), 24933642)
    }

    #[test]
    fn display_index() {
        let dic_storage = build_dic_tree_from_console(TEST_INP1);
        FSNode::display_index(&dic_storage, 0, 0)
    }
}