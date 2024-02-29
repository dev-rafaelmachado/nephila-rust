mod graph;
mod edge_list;

use graph::Graph;
use edge_list::EdgeList;

fn main() {
    let mut graph = EdgeList::new(false, false);

    graph.add_edge(1, 2, 10);
    graph.add_edge(2, 3, 30);

    if graph.is_neighbor(1, 2) {
        println!("1 is neighbor of 2");
    } else {
        println!("1 is not neighbor of 2");
    }
    if graph.is_neighbor(1, 3) {
        println!("1 is neighbor of 3");
    } else {
        println!("1 is not neighbor of 3");
    }


    graph.print();
}
