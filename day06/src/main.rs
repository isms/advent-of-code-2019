use std::io::{self, BufRead, Error};
use std::collections::HashMap;

fn orbits(key: &str, tree: &HashMap<String, String>) -> i32 {
    let mut counter = 1;
    let mut curr = key.clone();
    loop {
        if let Some(value) = tree.get(curr) {
            if value == "COM" {
                break;
            }
            counter += 1;
            curr = &*value;
        } else {
            break;
        }
    }
    counter
}

fn create_tree(entries: Vec<String>) -> HashMap<String, String> {
    let mut tree: HashMap<String, String> = HashMap::new();
    for line in entries.iter() {
        let parts: Vec<&str> = line.trim().split(")").collect();
        let child = parts[1].to_string();
        let parent = parts[0].to_string();
        tree.insert(child, parent);
    }
    tree
}

fn part1(tree: &HashMap<String, String>) -> i32 {
    let distances: Vec<i32> = tree.keys().cloned().into_iter().map(|k| orbits(&k, &tree)).collect();
    distances.iter().sum()
}

fn part2(tree: &HashMap<String, String>, from: &str, to: &str) -> i32 {
    return 14;
}

fn main() -> Result<(), Error> {
    let stdin: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let tree = create_tree(stdin);
    println!("{:?}", part1(&tree));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let small: Vec<String> = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L".lines().map(|x| x.to_owned()).collect();
        let tree = create_tree(small);
        assert_eq!(part1(&tree), 42)
    }

    #[test]
    fn test_part2() {
        let small: Vec<String> = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN".lines().map(|x| x.to_owned()).collect();
        let tree = create_tree(small);
        assert_eq!(part2(&tree, "YOU", "SAN"), 42)
    }
}