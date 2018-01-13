extern crate regex;
extern crate petgraph;

use regex::Regex;
use petgraph::Graph;
use petgraph::Undirected;
use petgraph::algo::kosaraju_scc;
use std::collections::HashMap;

fn get_input() -> Vec<Vec<String>> {
    const INPUT: &str = include_str!("input.txt");
    convert(INPUT)
}

fn convert(input: &str) -> Vec<Vec<String>> {
    let re = Regex::new("[[:word:]]+").unwrap();

    input.lines()
        .map(|line| re.find_iter(&line)
             .map(|program| program.as_str().to_string())
             .collect())
        .collect::<Vec<Vec<String>>>()
}

fn main() {
    let lines = get_input();
    let mut indices = HashMap::new();
    let mut graph = Graph::<String, (), Undirected>::new_undirected();

    // add all nodes
    for ref line in &lines {
        let program = line[0].to_string();
        let index = graph.add_node(program);
        indices.insert(line[0].to_string(), index);
        
    }

    // add edges
    for line in &lines {
        for child in &line[1..] {
            graph.add_edge(indices[&line[0]], indices[child], ());
        }
    }


    let connected_components = kosaraju_scc(&graph);

    let program_0_index = indices[&"0".to_string()];

    for component_set in &connected_components {
        if component_set.contains(&program_0_index) {
            println!("There are {} programs in the group containing program \"0\"", component_set.len());
        }
    }

    println!("There are a total of {} groups", connected_components.len());
}
