mod structure_types;
mod shared;

// use crate::structure_types::edge_list::EdgeList;
use crate::structure_types::adjacency_matrix::AdjacencyMatrix;
use crate::shared::graph::TGraph;

fn main() {
    let mut graph = AdjacencyMatrix::new(false, true);

    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");

    graph.add_edge("A", "B", 1);
    graph.add_edge("B", "C", 2);
    graph.add_edge("C", "D", 3);
    graph.add_edge("D", "A", 4);

    graph.print();

    let a_neighbors = graph.get_neighbors("A");
    println!("{:?}", a_neighbors);

    let b_and_c_is_neighbors = graph.is_neighbor("B", "C");
    println!("{}", b_and_c_is_neighbors);

    graph.remove_node("A");
    graph.remove_edge("B", "C");



    graph.print();
}