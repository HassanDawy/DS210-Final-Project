use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Graph {
    pub adj_list: HashMap<usize, HashSet<usize>>,
    pub num_nodes: usize,
    pub num_edges: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
            num_nodes: 0,
            num_edges: 0,
        }
    }

    pub fn load_from_file(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open graph file.");
        let reader = BufReader::new(file);
        let mut graph = Graph::new();

        for line in reader.lines() {
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


    pub fn all_degrees(&self) -> Vec<(usize, usize)> {
        self.adj_list
            .iter()
            .map(|(&node, neighbors)| (node, neighbors.len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_all_degrees() {
        let mut graph = Graph::new();
        graph.adj_list.insert(1, HashSet::from([2, 3]));
        graph.adj_list.insert(2, HashSet::from([1]));
        graph.adj_list.insert(3, HashSet::from([1]));
        graph.num_nodes = graph.adj_list.len();
        graph.num_edges = 2; 
        assert_eq!(graph.num_nodes, 3);
        assert_eq!(graph.num_edges, 2);

        let degrees = graph.all_degrees();
        assert!(degrees.contains(&(1, 2))); 
        assert!(degrees.contains(&(2, 1)));
        assert!(degrees.contains(&(3, 1))); 
    }
}