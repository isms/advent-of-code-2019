use std::io::{self, BufRead, Error};
use std::collections::HashMap;
use std::cmp;

type Tree = HashMap<String, String>;

fn orbits(key: &str, tree: &Tree) -> i32 {
    return path(key, tree).len() as i32;
}

fn path(key: &str, tree: &Tree) -> Vec<String> {
    let mut p: Vec<String> = vec![key.to_owned()];
    let mut curr = key.clone();
    loop {
        curr = tree.get(curr).unwrap();
        if curr == "COM" {
            return p;
        }
        p.push(curr.to_owned());
    }
}

fn create_tree(entries: Vec<String>) -> Tree {
    let mut tree: Tree = HashMap::new();
    for line in entries.iter() {
        let parts: Vec<&str> = line.trim().split(")").collect();
        let child = parts[1].to_string();
        let parent = parts[0].to_string();
        tree.insert(child, parent);
    }
    tree
}

fn part1(tree: &Tree) -> i32 {
    let distances: Vec<i32> = tree.keys().cloned().into_iter().map(|k| orbits(&k, &tree)).collect();
    distances.iter().sum()
}

fn part2(tree: &Tree, node1: &str, node2: &str) -> i32 {
    let h1: HashMap<String, i32> = path(node1, tree).iter().skip(1).enumerate().map(|(v, k)| (k.clone(), v as i32)).collect();
    let h2: HashMap<String, i32> = path(node2, tree).iter().skip(1).enumerate().map(|(v, k)| (k.clone(), v as i32)).collect();
    let mut min = i32::max_value();
    for k1 in h1.keys().clone().into_iter() {
        for k2 in h2.keys().clone().into_iter() {
            if *k1 == *k2 {
                min = cmp::min(min, h1.get(k1).unwrap() + h2.get(k2).unwrap());
            }
        }
    }
    min
}

fn main() -> Result<(), Error> {
    let stdin: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    let tree = create_tree(stdin);
    println!("part1: {:?}", part1(&tree));
    println!("part2: {:?}", part2(&tree, "YOU", "SAN"));
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
        assert_eq!(path("H", &tree), vec!["H".to_owned(), "G".to_owned(), "B".to_owned()]);
        assert_eq!(part1(&tree), 42);
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
        assert_eq!(part2(&tree, "YOU", "SAN"), 4);
    }
}