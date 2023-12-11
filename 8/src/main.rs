use std::{collections::HashMap, error::Error};

// struct Node<'a> {
//     name: &'a str,
//     left: &'a str,
//     right: &'a str,
// }
//

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
//

// fn get_all_nodes_for_dir() -> Vec<(&&str, &(&str, &str))> {
//
//
// }

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    if a == 0 {
        return b;
    }

    if a > b {
        return gcd(b, a % b);
    } else {
        return gcd(a, b % a);
    }
}

fn lcm(a: usize, b: usize) -> usize {
    return (a * b) / gcd(a, b);
}

type Node<'a> = (&'a &'a str, &'a (&'a str, &'a str));

fn get_num_steps<'a>(
    start_node: Node<'a>,
    instructions: &Vec<char>,
    nodes: &HashMap<&str, (&str, &str)>,
    check: &str,
) -> usize {
    let mut node_to_check = start_node;
    let mut finished = false;
    let mut steps = 0;

    while !finished {
        instructions.iter().for_each(|instruction| {
            match instruction {
                'L' => {
                    let left_node_name = node_to_check.1 .0;
                    node_to_check = nodes
                        .get_key_value(left_node_name)
                        .expect(&format!("can't find node: {}", left_node_name))
                }
                'R' => {
                    let right_node_name = node_to_check.1 .1;
                    node_to_check = nodes
                        .get_key_value(right_node_name)
                        .expect(&format!("can't find node: {}", right_node_name))
                }
                _ => panic!("unknown instruction"),
            }
            if node_to_check.0.ends_with(check) {
                finished = true;
            }
            steps += 1;
        })
    }

    return steps;
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = std::fs::read_to_string("input").expect("couldn't read file");

    let instructions = file_content
        .lines()
        .nth(0)
        .expect("could not read first line")
        .chars()
        .collect::<Vec<_>>();

    let nodes = file_content
        .lines()
        .skip(1)
        .filter_map(|line| {
            let (node, left_right) = line.split_once("=")?;
            let node = node.trim();

            let (left, right) = left_right
                .trim()
                .strip_prefix("(")?
                .strip_suffix(")")?
                .split_once(",")?;

            return Some((node.trim(), (left.trim(), right.trim())));
        })
        .collect::<HashMap<_, _>>();

    let node_to_check = nodes.get_key_value("AAA").ok_or("can't find start node")?;
    let steps = get_num_steps(node_to_check, &instructions, &nodes, "ZZZ");
    println!("part 1 : {}", steps);

    let nodes_to_check: Vec<_> = nodes
        .iter()
        .filter(|(name, _path)| name.ends_with("A"))
        .collect();

    let steps = nodes_to_check
        .into_iter()
        .map(|node| get_num_steps(node, &instructions, &nodes, "Z"))
        .reduce(|acc, e| {
            return lcm(acc, e);
        });

    println!("part 2 : {}", steps.unwrap());

    Ok(())
}

#[test]
fn lcm_test() {
    let result = lcm(21, 6);
    assert_eq!(result, 42);
}
