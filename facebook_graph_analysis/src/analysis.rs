//Module: analysis.rs
//Here we implement graph algorithms that will analyze social connectivity and structural similarity
use std::collections::{HashMap, HashSet, VecDeque};
use crate::graph::Graph;

pub fn average_distance(graph: &Graph) -> f64 { // Computes the average distance between all reachable node pairs in the graph.
    let mut total_distance = 0usize;
    let mut count = 0usize;

    for &start in graph.adj_list.keys() { //For each node, use BFS (defined in the function after this) to compute distances to all others. 
        let distances = bfs_distances(graph, start);
        for &dist in distances.values() {
            if dist > 0 {
                total_distance += dist;
                count += 1;
            }
        }
    }

    if count == 0 { 0.0 } else { total_distance as f64 / count as f64 }
}

pub fn bfs_distances(graph: &Graph, start: usize) -> HashMap<usize, usize> { //Performs Breadth-First Search (BFS) from a start node.
    let mut visited = HashSet::new();
    let mut distance = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    distance.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() { // Uses a queue and a visited set to explore each level of the graph
        let current_dist = distance[&current];
        if let Some(neighbors) = graph.adj_list.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    distance.insert(neighbor, current_dist + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    distance
}

pub fn closeness_centrality(graph: &Graph) -> Vec<(usize, f64)> { //Computes closeness centrality for all nodes in the graph.
    let mut result = vec![];
    for &node in graph.adj_list.keys() { //For each node, sum shortest path distances through bfs, then compute inverse. if theres higher closeness then its a more central node.
        let dist = bfs_distances(graph, node);
        let sum: usize = dist.values().sum();
        let closeness = if sum > 0 { //Avoiding division by 0
            (dist.len() - 1) as f64 / sum as f64
        } else {
            0.0
        };
        result.push((node, closeness));
    }
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    result
}

pub fn jaccard_similarity(graph: &Graph, u: usize, v: usize) -> f64 { //Computes the Jaccard similarity between two nodes in the graph - measures social similarity based on mutual friends
    let a = graph.adj_list.get(&u);
    let b = graph.adj_list.get(&v);

    match (a, b) {
        (Some(set1), Some(set2)) => {
            let intersection = set1.intersection(set2).count() as f64;
            let union = set1.union(set2).count() as f64;
            if union == 0.0 { 0.0 } else { intersection / union }
        }
        _ => 0.0,
    }
}

pub fn most_similar_pairs(graph: &Graph, top_n: usize) -> Vec<((usize, usize), f64)> { //Computes top N most similar node pairs based on Jaccard similarity.
    let mut results = Vec::new();
    let nodes: Vec<usize> = graph.adj_list.keys().copied().collect();

    for i in 0..nodes.len() { //For all unique node pairs it will compute similarity, skip sparse nodes, and sort them
        for j in i + 1..nodes.len() {
            let u = nodes[i];
            let v = nodes[j];
            let neighbors_u = graph.adj_list.get(&u);
            let neighbors_v = graph.adj_list.get(&v);
            if neighbors_u.map_or(true, |n| n.len() <= 1) || neighbors_v.map_or(true, |n| n.len() <= 1) {
                continue;
            }
            let sim = jaccard_similarity(graph, u, v);
            if sim > 0.0 {
                results.push(((u, v), sim));
            }
        }
    }

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results.truncate(top_n);
    results
}

//TESTS for the algorithms in analysis.rs
#[cfg(test)] //To include our tests in cargo test
mod tests { //Submodule to put our tests in
    use super::*;
    use crate::graph::Graph;
    use std::collections::HashSet;
    fn small_graph() -> Graph { //Creates a simple triangle graph to test our algorithms on
        let mut graph = Graph::new();
        graph.adj_list.insert(0, HashSet::from([1, 2]));
        graph.adj_list.insert(1, HashSet::from([0, 2]));
        graph.adj_list.insert(2, HashSet::from([0, 1]));
        graph.num_nodes = 3;
        graph.num_edges = 3;
        graph
    }

    #[test]
    fn test_bfs_distances() { //Verifies that BFS correctly computes the shortest distances from node 0.
        let graph = small_graph();
        let distances = bfs_distances(&graph, 0);
        assert_eq!(distances.get(&0), Some(&0));
        assert_eq!(distances.get(&1), Some(&1));
        assert_eq!(distances.get(&2), Some(&1));
    }

    #[test]
    fn test_average_distance() { //Checks that the average distance computed is accurate for a fully connected triangle graph
        let graph = small_graph();
        let avg_dist = average_distance(&graph);
        assert!((avg_dist - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_closeness_centrality() { //Ensures closeness centrality returns 1.0 for all nodes in our closed trianlge
        let graph = small_graph();
        let closeness = closeness_centrality(&graph);
        for &(_, centrality) in &closeness {
            assert!((centrality - 1.0).abs() < 0.0001);
        }
    }

    #[test]
    fn test_jaccard_similarity() { //Validates Jaccard similarity between two nodes with shared neighbors in the triangle.
        let graph = small_graph();
        let sim = jaccard_similarity(&graph, 0, 1);
        assert!((sim - (1.0 / 3.0)).abs() < 0.0001);
    }
}
