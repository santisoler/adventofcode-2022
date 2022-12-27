use std::fs::File;
use std::io::Read;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_visible_from_right() {
        let fname = String::from("data/test_input");
        let forest = parse_file_to_2d_array(&read_file(&fname));
        assert_eq!(is_visible_from_right(&forest, &1, &1), false);
        assert_eq!(is_visible_from_right(&forest, &1, &2), true);
    }

    #[test]
    fn test_is_visible_from_left() {
        let fname = String::from("data/test_input");
        let forest = parse_file_to_2d_array(&read_file(&fname));
        assert_eq!(is_visible_from_left(&forest, &2, &2), false);
        assert_eq!(is_visible_from_left(&forest, &3, &2), true);
    }

    #[test]
    fn test_is_visible_from_up() {
        let fname = String::from("data/test_input");
        let forest = parse_file_to_2d_array(&read_file(&fname));
        assert_eq!(is_visible_from_up(&forest, &1, &3), false);
        assert_eq!(is_visible_from_up(&forest, &1, &2), true);
    }

    #[test]
    fn test_is_visible_from_down() {
        let fname = String::from("data/test_input");
        let forest = parse_file_to_2d_array(&read_file(&fname));
        assert_eq!(is_visible_from_down(&forest, &2, &2), false);
        assert_eq!(is_visible_from_down(&forest, &3, &2), true);
    }

    #[test]
    fn test_is_tree_visible() {
        let fname = String::from("data/test_input");
        let forest = parse_file_to_2d_array(&read_file(&fname));
        assert_eq!(is_tree_visible(&forest, &1, &1), true);
    }

    #[test]
    fn test_part1() {
        let fname = String::from("data/test_input");
        let result = solve_part1(&fname);
        assert_eq!(result, 21);
    }

    // #[test]
    // fn test_part2() {
    //     let fname = String::from("data/test_input");
    //     let result = solve_part2(&fname);
    //     assert_eq!(result, 24933642);
    // }
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

fn parse_file_to_2d_array(file_content: &str) -> Vec<Vec<u32>> {
    // Parse file content into a 2D vector
    let mut forest: Vec<Vec<u32>> = vec![];
    for line in file_content.lines() {
        let row: Vec<u32> = line
            .chars()
            .map(|c| String::from(c).parse().unwrap())
            .collect();
        forest.push(row);
    }
    forest
}

fn is_tree_visible(forest: &Vec<Vec<u32>>, row: &usize, column: &usize) -> bool {
    // Check if a tree is visible from the edge along the four directions
    if is_visible_from_up(&forest, &row, &column)
        || is_visible_from_down(&forest, &row, &column)
        || is_visible_from_left(&forest, &row, &column)
        || is_visible_from_right(&forest, &row, &column)
    {
        return true;
    }
    false
}

fn is_visible_from_right(forest: &Vec<Vec<u32>>, row: &usize, column: &usize) -> bool {
    // Check if a tree is visible from the right edge
    let ncolumns = forest[*row].len();
    for j in column + 1..ncolumns {
        if forest[*row][*column] <= forest[*row][j] {
            return false;
        }
    }
    true
}

fn is_visible_from_left(forest: &Vec<Vec<u32>>, row: &usize, column: &usize) -> bool {
    // Check if a tree is visible from the left edge
    for j in 0..*column {
        if forest[*row][*column] <= forest[*row][j] {
            return false;
        }
    }
    true
}

fn is_visible_from_up(forest: &Vec<Vec<u32>>, row: &usize, column: &usize) -> bool {
    // Check if a tree is visible from the upmost edge
    for i in 0..*row {
        if forest[*row][*column] <= forest[i][*column] {
            return false;
        }
    }
    true
}

fn is_visible_from_down(forest: &Vec<Vec<u32>>, row: &usize, column: &usize) -> bool {
    // Check if a tree is visible from the downmost edge
    let nrows = forest.len();
    for i in row + 1..nrows {
        if forest[*row][*column] <= forest[i][*column] {
            return false;
        }
    }
    true
}

fn solve_part1(fname: &String) -> u32 {
    // Read data file
    let data = read_file(&fname);
    // Parse input file
    let forest = parse_file_to_2d_array(&data);
    // Initialize number of visible trees. Count the tress that lie in the edge
    let nrows = forest.len();
    let ncolumns = forest[0].len();
    let mut n_visible_trees = 2 * (nrows + ncolumns) as u32 - 4;
    for i in 1..nrows - 1 {
        for j in 1..ncolumns - 1 {
            if is_tree_visible(&forest, &i, &j) {
                n_visible_trees += 1;
            };
        }
    }
    n_visible_trees
}

fn main() {
    let fname = String::from("data/input");
    // let fname = String::from("data/test_input");

    // part 1
    let result = solve_part1(&fname);
    println!("Solution to part 1: {}", result);

    // // part 2
    // let result = solve_part2(&fname);
    // println!("Solution to part 2: {}", result);
}
