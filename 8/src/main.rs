extern crate read_input;

use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    children: Vec<usize>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(children: Vec<usize>, metadata: Vec<usize>) -> Self {
        Node { children, metadata }
    }
}

fn insert_node(
    numbers: &Vec<usize>,
    tree: &mut HashMap<usize, Node>,
    metadata_starting_point: usize,
    metadata_count: usize,
    id: &mut usize,
    child_ids: Vec<usize>,
) {
    let iter = numbers.iter().skip(metadata_starting_point);
    let metadata: Vec<usize> = iter.take(metadata_count).cloned().collect();
    *id += 1;
    tree.insert(*id, Node::new(child_ids, metadata));
}

// returns (next_index, metadata_count, child_ids)
fn parse_next_node(
    numbers: &Vec<usize>,
    tree: &mut HashMap<usize, Node>,
    node_starting_point: usize,
    id: &mut usize,
) -> (usize, usize, Vec<usize>) {
    let children_count = numbers.get(node_starting_point).unwrap();
    let metadata_count = numbers.get(node_starting_point + 1).unwrap();

    if *children_count == 0 {
        return (
            node_starting_point + metadata_count + 2,
            *metadata_count,
            Vec::new(),
        );
    }

    let mut next_node_starting_point = node_starting_point + 2;
    let mut end_of_children = 0;

    let mut child_ids = Vec::new();
    for _ in 0..*children_count {
        {
            let (end_of_child, metadata_count, child_ids) =
                parse_next_node(numbers, tree, next_node_starting_point, id);
            insert_node(
                numbers,
                tree,
                end_of_child - metadata_count,
                metadata_count,
                id,
                child_ids,
            );
            next_node_starting_point = end_of_child;
            end_of_children = end_of_child;
        }

        child_ids.push(*id);
    }

    (end_of_children + metadata_count, *metadata_count, child_ids)
}

fn find_root_value(tree: &HashMap<usize, Node>, id: usize) -> usize {
    let node = tree.get(&id).unwrap();
    if node.children.len() == 0 {
        let sum = node.metadata.iter().fold(0, |sum, n| sum + n);
        sum
    } else {
        node.metadata
            .iter()
            .filter(|index| node.children.get(**index - 1).is_some())
            .map(|idx| find_root_value(tree, *node.children.get(*idx - 1).unwrap()))
            .fold(0, |sum, n| sum + n)
    }
}

fn main() {
    let text = read_input::read_text("8/input.txt").unwrap();
    let numbers: Vec<usize> = text.split(" ").map(|n| n.parse().unwrap()).collect();

    let node_starting_point = 0;
    let mut tree = HashMap::new();

    let mut id = 1;

    let (end_of_child, metadata_count, child_ids) =
        parse_next_node(&numbers, &mut tree, node_starting_point, &mut id);
    insert_node(
        &numbers,
        &mut tree,
        end_of_child - metadata_count,
        metadata_count,
        &mut id,
        child_ids,
    );

    let sum = tree.iter().fold(0, |total, (_, node)| {
        total + node.metadata.iter().fold(0, |sum, n| sum + n)
    });

    println!("{}", sum);
    println!("{}", find_root_value(&tree, id));
}
