use crate::shared::node::Node;
use crate::shared::graph::TGraph;
use crate::shared::graph::TNode;

pub struct AdjacencyList {
    nodes: Vec<Box<Node>>,
    is_directed: bool,
    is_weighted: bool,
}

impl TGraph for AdjacencyList {
    fn new(is_directed: bool, is_weighted: bool) -> Self {
        AdjacencyList {
            nodes: Vec::new(),
            is_directed,
            is_weighted,
        }
    }

    fn add_node(&mut self, label: &str) {
        self.nodes.push(Box::new(Node::new(label)));
    }

    fn add_edge(&mut self, from: &str, to: &str, weight: i32) {
        let from_node = self.nodes.iter_mut().find(|node| node.get_value() == from).unwrap();
        let weight = if self.is_weighted { weight } else { 1 };
        from_node.add_neighbor(to, weight);
        if !self.is_directed {
            let to_node = self.nodes.iter_mut().find(|node| node.get_value() == to).unwrap();
            to_node.add_neighbor(from, weight);
        }
    }

    fn remove_node(&mut self, label: &str) {
        self.nodes.retain(|node| node.get_value() != label);
        for node in &mut self.nodes {
            node.remove_neighbor(label);
        }
    }

    fn remove_edge(&mut self, from: &str, to: &str) {
        let from_node = self.nodes.iter_mut().find(|node| node.get_value() == from).unwrap();
        from_node.remove_neighbor(to);
        if !self.is_directed {
            let to_node = self.nodes.iter_mut().find(|node| node.get_value() == to).unwrap();
            to_node.remove_neighbor(from);
        }
    }

    fn is_neighbor(&mut self, from: &str, to: &str) -> bool {
        let from_node = self.nodes.iter_mut().find(|node| node.get_value() == from).unwrap();
        from_node.is_neighbor(to)
    }

    fn get_neighbors(&self, label: &str) -> Vec<(&str, i32)> {
        let node = self.nodes.iter().find(|node| node.get_value() == label).unwrap();
        node.get_neighbors()
    }

    fn print(&self) {
        for node in &self.nodes {
            node.print();
        }
    }
}

    