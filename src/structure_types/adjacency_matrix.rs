use crate::shared::node::Node;
use crate::shared::graph::TGraph;
use crate::shared::graph::TNode;

pub struct AdjacencyMatrix {
    matrix: Vec<Vec<i32>>,
    nodes: Vec<Box<Node>>,
    is_directed: bool,
    is_weighted: bool,
}

impl TGraph for AdjacencyMatrix {
    fn new(is_directed: bool, is_weighted: bool) -> Self {
        AdjacencyMatrix {
            matrix: Vec::new(),
            nodes: Vec::new(),
            is_directed,
            is_weighted,
        }
    }

    fn add_node(&mut self, label: &str) {
        
        self.nodes.iter().for_each(|n| {
            if n.get_value() == label {
                panic!("Node already exists");
            }
        });

        self.nodes.push(Box::new(Node::new(label)));
        
        let n = self.nodes.len();
        for row in self.matrix.iter_mut() {
            row.push(0);
        }
        self.matrix.push(vec![0; n + 1]);
    }

    fn add_edge(&mut self, from: &str, to: &str, weight: i32) {
        let from_index = self.nodes.iter().position(|n| n.get_value() == from).expect("Node not found");
        let to_index = self.nodes.iter().position(|n| n.get_value() == to).expect("Node not found");
        
        let weight = if self.is_weighted { weight } else { 1 };
        
        self.matrix[from_index][to_index] = if self.is_weighted { weight } else { 1 };
        if !self.is_directed {
            self.matrix[to_index][from_index] = weight;
        }
    }

    fn remove_node(&mut self, label: &str) {
        let index = self.nodes.iter().position(|n| n.get_value() == label).expect("Node not found");
        self.nodes.remove(index);
        self.matrix.remove(index);
        self.matrix.iter_mut().for_each(|row| { row.remove(index); });
    }

    fn remove_edge(&mut self, from: &str, to: &str) {
        let from_index = self.nodes.iter().position(|n| n.get_value() == from).expect("Node not found");
        let to_index = self.nodes.iter().position(|n| n.get_value() == to).expect("Node not found");
        self.matrix[from_index][to_index] = 0;
        if !self.is_directed {
            self.matrix[to_index][from_index] = 0;
        }
    }

    fn is_neighbor(&mut self, from: &str, to: &str) -> bool {
        let from_index = self.nodes.iter().position(|n| n.get_value() == from).expect("Node not found");
        let to_index = self.nodes.iter().position(|n| n.get_value() == to).expect("Node not found");
        self.matrix[from_index][to_index] != 0
    }

    fn get_neighbors(&self, label: &str) -> Vec<(&str, i32)> {
        let index = self.nodes.iter().position(|n| n.get_value() == label).expect("Node not found");
        self.nodes.iter().enumerate().filter_map(|(i, n)| {
            if self.matrix[index][i] != 0 {
                Some((n.get_value(), self.matrix[index][i]))
            } else {
                None
            }
        }).collect()
    }

    fn print(&self) {
        println!("Adjacency Matrix:");
        for row in &self.matrix {
            println!("{:?}", row[0..self.nodes.len()].to_vec());
        }
    }

}