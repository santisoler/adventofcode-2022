use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parent() {
        let directory = String::from("/usr/share/lib");
        assert_eq!(get_parent(&directory), String::from("/usr/share"));
        let directory = String::from("/usr");
        assert_eq!(get_parent(&directory), String::from("/"));
        let directory = String::from("/a/b/c/d");
        assert_eq!(get_parent(&directory), String::from("/a/b/c"));
        let directory = String::from("/a");
        assert_eq!(get_parent(&directory), String::from("/"));
    }

    #[test]
    fn test_get_all_parents() {
        let directory = String::from("/usr/share/lib");
        let expected = vec![
            String::from("/usr/share"),
            String::from("/usr"),
            String::from("/"),
        ];
        assert_eq!(get_all_parents(&directory), expected);
        let directory = String::from("/");
        let expected: Vec<&str> = vec![];
        assert_eq!(get_all_parents(&directory), expected);
        let directory = String::from("/a/b/c/d/e");
        let expected = vec![
            String::from("/a/b/c/d"),
            String::from("/a/b/c"),
            String::from("/a/b"),
            String::from("/a"),
            String::from("/"),
        ];
        assert_eq!(get_all_parents(&directory), expected);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part2() {
        let fname = String::from("data/test_input");
        let result = solve_part2(&fname);
        assert_eq!(result, 24933642);
    }
}

fn read_file(fname: &String) -> String {
    // Open file
    let path = Path::new(&fname);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", fname, why),
        Ok(file) => file,
    };
    // Parse file
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => panic!("couldn't read {}: {}", fname, why),
        Ok(_) => (),
    };
    content
}

fn get_parent(directory: &str) -> &str {
    // Return the path to the parent dir of the given directory
    let mut index: usize = 0;
    for (i, c) in directory.chars().enumerate() {
        if c == '/' {
            index = i;
        }
    }
    // If the parent directory is /, return a slice with the first character
    if index == 0 {
        return &directory[..1];
    }
    return &directory[..index];
}

fn get_all_parents(cwd: &str) -> Vec<&str> {
    // Return all parents dir of cwd
    let mut parents: Vec<&str> = Vec::new();
    let mut parent = cwd;
    while parent != String::from("/") {
        parent = get_parent(&parent);
        parents.push(parent)
    }
    parents
}

fn get_size_of_directories(file_content: &str) -> HashMap<String, u32> {
    // Return a hasmap with the size of every directory in the tree
    //
    // Define a hashmap for the size of the directories
    let mut directories: HashMap<String, u32> = HashMap::new();
    // Define a variable to store the current directory
    let mut cwd = String::new();
    // Parse the input file into a hashmap that contain the size of each dir
    for line in file_content.lines() {
        // Ignore lines that run ls
        if line == "$ ls" {
            continue;
        };
        // Change cwd if line runs the cd command
        if line[0..4] == String::from("$ cd") {
            let new_dir = line.split_whitespace().last().unwrap();
            if new_dir == String::from("..") {
                cwd = get_parent(&cwd).to_owned();
            } else {
                if new_dir == String::from("/") {
                    cwd = String::from("/");
                } else if cwd == String::from("/") {
                    cwd = format!("/{}", new_dir);
                } else {
                    cwd = format!("{}/{}", cwd, new_dir);
                }
                if !directories.contains_key(&cwd) {
                    directories.insert(cwd.clone(), 0);
                }
            }
            continue;
        };

        // Read stdout lines
        if line[0..3] == String::from("dir") {
            continue;
        }

        // Add file size to cwd in directories hashmap
        let file_size: u32 = line.split_whitespace().nth(0).unwrap().parse().unwrap();
        directories
            .entry(cwd.clone())
            .and_modify(|s| *s += file_size);
        // And add it to every parent of cwd
        for parent in get_all_parents(&cwd).iter() {
            directories
                .entry(String::from(*parent).clone())
                .and_modify(|s| *s += file_size);
        }
    }
    directories
}

fn solve_part1(fname: &String) -> u32 {
    // Read data file
    let file_content = read_file(&fname);
    // Get size of directories
    let directories = get_size_of_directories(&file_content);
    // Compute the sum of all directories sizes at most as 100000
    let mut result: u32 = 0;
    for (_, size) in directories.iter() {
        if *size <= 100000 as u32 {
            result += size
        }
    }
    result
}

fn solve_part2(fname: &String) -> u32 {
    // Read data file
    let file_content = read_file(&fname);
    // Get size of directories
    let directories = get_size_of_directories(&file_content);
    // Define varibales for total size of the drive, the required space for the update and the
    // current size of the root.
    let total_size: u32 = 70_000_000;
    let required_space: u32 = 30_000_000;
    let size_of_root = directories.get(&String::from("/")).unwrap();
    // Calculate the minimum size that the directory should have in order to be a candidate for
    // deletion
    let min_size = size_of_root + required_space - total_size;
    // Find the smallest directory that we can delete to free enough space
    let mut result: u32 = total_size;
    for (_, size) in directories.iter() {
        if *size >= min_size && *size < result {
            result = *size
        }
    }
    result
}

fn main() {
    let fname = String::from("data/input");
    // let fname = String::from("data/test_input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // // part 2
    let result = solve_part2(&fname);
    println!("Solution to part 2: {}", result);
}
