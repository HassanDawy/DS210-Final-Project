//Entry point of our project for analyzing the graph
//Generally, we load our graph, and perform our algorithms on it
//REMARK MY Write-Up goes over all the code very extensively and goes over the output there too! make sure to check that out
mod graph; //Module that defines and builds the Graph structure
mod analysis; //Module that implements analysis algorithms
use graph::Graph;
use analysis::{average_distance, closeness_centrality, jaccard_similarity, most_similar_pairs};

fn main() {
    //Loading graph data
    let path = "data/facebook_combined.txt";
    let graph = Graph::load_from_file(path);
    println!("Loaded {} nodes and {} edges.", graph.num_nodes, graph.num_edges);

    //Print the degree of the first 10 nodes
    println!("\nDegree Distribution:");
    for (node, degree) in graph.all_degrees().iter().take(10) {
        println!("Node {:>4}: Degree {:>3}", node, degree);
    }
    println!("_____________");

    //Compute and display the average shortest path length
    let avg_dist = average_distance(&graph);
    println!("\nAverage Distance (Six Degrees): {:.2}", avg_dist);
    println!("_____________");

    //Compute and display top 5 nodes ranked by closeness centrality
    println!("\nTop 5 Closeness Centrality Nodes:");
    for (node, centrality) in closeness_centrality(&graph).into_iter().take(5) {
        println!("Node {:>4}: Closeness Centrality {:.4}", node, centrality);
    }
    println!("_____________");

    //Compute and print Jaccard similarity for selected friend pairs
    let pairs = vec![(0, 1), (0, 2), (1, 3)];
    println!("\nJaccard Similarities (Friends of Friends):");
    for (u, v) in pairs {
        let sim = jaccard_similarity(&graph, u, v);
        println!("Nodes {} & {} → Similarity: {:.3}", u, v, sim);
    }
    println!("_____________");

    //Find and print the top 5 most similar node pairs in the entire graph
    println!("\nTop Jaccard Similarities (Most Similar Friend Pairs):");
    for ((u, v), sim) in most_similar_pairs(&graph, 5) {
        println!("Nodes {} & {} → Similarity: {:.3}", u, v, sim);
    }

    //Debugging: Check friends of a reference node to verify similarity behavior (making sure jaccard is working)
    let reference = 2817;
    if let Some(friends) = graph.adj_list.get(&reference) {
        println!("Node {} has {} friends: {:?}", reference, friends.len(), friends);
}
}