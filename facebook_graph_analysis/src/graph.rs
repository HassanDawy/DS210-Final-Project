//Module: graph.rs
//Here we define the graph struct and build the graph to prepare us for analysis methods

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Graph { // We represent an undirected graph using an adjacency list to model our social network
    pub adj_list: HashMap<usize, HashSet<usize>>,
    pub num_nodes: usize,
    pub num_edges: usize,
}

impl Graph {
    pub fn new() -> Self { //Creates a new empty graph with zero nodes and edges
        Self {
            adj_list: HashMap::new(),
            num_nodes: 0,
            num_edges: 0,
        }
    }

    pub fn load_from_file(path: &str) -> Self { //We load a graph from a file where each line represents an edge as "u", "v" It reads each file line by line, parses each edge, and builds the adjacency list
        let file = File::open(path).expect("Failed to open graph file.");
        let reader = BufReader::new(file);
        let mut graph = Graph::new();

        for line in reader.lines() { //Reading edges line by line
            if let Ok(edge_line) = line {
                let parts: Vec<usize> = edge_line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                if parts.len() != 2 {
                    continue;
                }
                let (u, v) = (parts[0], parts[1]);
                graph.adj_list.entry(u).or_default().insert(v);
                graph.adj_list.entry(v).or_default().insert(u);
                graph.num_edges += 1;
            }
        }

        graph.num_nodes = graph.adj_list.len();
        graph
    }

    // Computes the degree (number of neighbors) for each node in the graph
    // Output: Vector of (node, degree) pairs
    pub fn all_degrees(&self) -> Vec<(usize, usize)> {
        self.adj_list
            .iter()
            .map(|(&node, neighbors)| (node, neighbors.len()))
            .collect()
    }
}


//TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_all_degrees() { //We test the degree computation function with a small graph
        let mut graph = Graph::new();
        graph.adj_list.insert(1, HashSet::from([2, 3]));
        graph.adj_list.insert(2, HashSet::from([1]));
        graph.adj_list.insert(3, HashSet::from([1]));
        graph.num_nodes = graph.adj_list.len();
        graph.num_edges = 2; 
        assert_eq!(graph.num_nodes, 3);
        assert_eq!(graph.num_edges, 2);

        let degrees = graph.all_degrees();
        //Expected: node 1 has degree 2, nodes 2 and 3 have degree 1
        assert!(degrees.contains(&(1, 2))); 
        assert!(degrees.contains(&(2, 1)));
        assert!(degrees.contains(&(3, 1))); 
    }
}