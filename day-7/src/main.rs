#[macro_use]
extern crate nom;

use nom::{alpha, digit};
use std::collections::HashMap;

fn get_input() -> HashMap<String, Node> {
    const INPUT: &str = include_str!("input.txt");
    convert(INPUT)
}

named!(int<i32>,
    map_res!(
        map_res!(
            digit,
            std::str::from_utf8),
        std::str::FromStr::from_str));

named!(string<String>,
    map_res!(
        map_res!(
            alpha,
            std::str::from_utf8),
        std::str::FromStr::from_str));

named!(children<Vec<String>>,
    do_parse!(
        opt!(complete!(tag!(") -> "))) >>
        children: many0!(child) >>
        (children)
    )
);

named!(child<String>,
    do_parse!(
        child: string >>
        opt!(complete!(tag!(", "))) >>
        (child)));

named!(input_parser<(String, i32, Vec<String>)>,
    do_parse!(
        id: string >>
        tag!(" (") >>
        weight: int >>
        children: children >>
        (id, weight, children)
    )
);

fn convert(input: &str) -> HashMap<String, Node> {
    let mut result = input.lines()
        .filter_map(|line| input_parser(line.as_bytes()).to_result().ok())
        .map(|(id, weight, children)| (id, Node {parent: None, weight: weight, children: children}))
        .collect::<HashMap<String, Node>>();

    let keys = result.iter_mut()
        .map(|(id, node)| (id.clone(), node.children.clone()))
        .collect::<HashMap<String, Vec<String>>>();

    for (parent_id, children) in keys {
        for child_id in children {
            let child = result.get_mut(&child_id).unwrap();
            child.parent = Some(parent_id.to_string());
        }
    }

    result
}



fn main() {
    let graph = get_input();

    // random starting place
    let mut root_id = graph.keys().next().unwrap().to_string();
    // walk up the tree until the root is found
    while let Some(ref parent_id) = graph.get(&root_id).unwrap().parent {
        root_id = parent_id.to_string();
    }
    println!("Tree root is: {}", root_id);

    let mut total_weights: HashMap<String, i32> = graph.iter()
        .map(|(id, node)| (id.to_string(), node.weight))
        .collect();

    for node in graph.values() {
        let mut id_option = &node.parent; 
        // add the weight to all parents of the node
        while let Some(ref id) = *id_option {
            let total_weight = total_weights.get_mut(id).unwrap();
            *total_weight += node.weight;

            let current_node = graph.get(id).unwrap();
            id_option = &current_node.parent;
        }
    }

    for (id, node) in graph.iter() {
        if node.children.is_empty() {
            continue;
        }

        let ((diff_weight, diff_id), (norm_weight, norm_id)) = node.children.iter()
            .map(|item| {
                return item})
            .fold(((0, ""), (0, "")), |((diff_weight, diff_id), (prev_weight, prev_id)), new_id| {
                // fold to find the one weight that is not equal to the others
                let new_weight = *total_weights.get(new_id).unwrap();
                if prev_weight == 0 {
                    return ((new_weight, new_id), (new_weight, new_id));
                }
                if new_weight == prev_weight {
                    return ((diff_weight, diff_id), (new_weight, new_id));
                }
                if new_weight == diff_weight {
                    return ((prev_weight, prev_id), (new_weight, new_id));
                }
                return ((new_weight, new_id), (prev_weight, prev_id));
            });

        let difference = norm_weight - diff_weight;
        if difference != 0 {
            println!("Found unbalance in {}", id);
            for child_id in node.children.iter() {
                let child = graph.get(child_id).unwrap();
                let child_total_weight = total_weights.get(child_id).unwrap();
                println!("\t{}: {} (own weight: {})", child_id, child_total_weight, child.weight);
            }
            let new_weight = difference + graph.get(diff_id).unwrap().weight;
            println!("adjust weight of {} by {} to {}", diff_id, difference, new_weight);
        }
    }
}

pub struct Node {
    pub parent: Option<String>,
    pub weight: i32,
    pub children: Vec<String>,
}
