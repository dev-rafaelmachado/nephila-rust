// > Module: edge_list
use crate::shared::node::Node;
use crate::shared::graph::TGraph;
use crate::shared::graph::TNode;


pub struct EdgeList {
    // from, to, weight
    edges: Vec<(Box<Node>, Box<Node>, i32)>,
    is_directed: bool,
    is_weighted: bool,
}

impl TGraph for EdgeList {
    fn new(is_directed: bool, is_weighted: bool) -> Self {
        EdgeList {
            edges: Vec::new(),
            is_directed,
            is_weighted,
        }
    }

    fn add_node(&mut self, label: &str) {
        println!("{}", label);
        // not needed
    }

    fn add_edge(&mut self, from: &str, to: &str, weight: i32) {
        let from_node = Box::new(Node::new(from));
        let to_node = Box::new(Node::new(to));

        let i_weight = match self.is_weighted {
            true => weight,
            false => 1,
        };
        
        match self.is_directed {
            true => {
                self.edges.push((from_node, to_node, i_weight));
            }
            false => {
                self.edges.push((from_node.clone(), to_node.clone(), i_weight));
                self.edges.push((to_node, from_node, i_weight));
            }
        }
    }

    fn remove_node(&mut self, label: &str) {
        self.edges.retain(|(from, to, _)| {
            from.get_value() != label && to.get_value() != label
        });
    }

    fn remove_edge(&mut self, from: &str, to: &str) {
        self.edges.retain(|(f, t, _)| {
            f.get_value() != from && t.get_value() != to
        });
    }

    fn is_neighbor(&mut self, from: &str, to: &str) -> bool {
        self.edges.iter().any(|(f, t, _)| {
            f.get_value() == from && t.get_value() == to
        })
    }

    fn get_neighbors(&self, label: &str) -> Vec<(&str, i32)> {
        self.edges.iter().filter_map(|(f, t, w)| {
            if f.get_value() == label {
                Some((t.get_value(), *w))
            } else {
                None
            }
        }).collect()
    }

    fn print(&self) {
        for (from, to, weight) in &self.edges {
            println!("From: {}, To: {}, Weight: {}", from.get_value(), to.get_value(), weight);
        }
    }
} 
