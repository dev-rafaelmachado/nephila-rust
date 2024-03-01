use crate::shared::graph::TNode;
pub struct Node {
    value: String,
    neighbors: Option<Vec<Box<Node>>>,
}

impl TNode for Node {
    fn new(value: &str) -> Self {
        Node {
            value: value.to_string(),
            neighbors: None,
        }
    }

    fn new_with_neighbors(value: &str, neighbors: Vec<Box<Node>>) -> Self {
        Node {
            value: value.to_string(),
            neighbors: Some(neighbors),
        }
    }

    fn get_value(&self) -> &str {
        &self.value
    }

    fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    fn add_neighbor(&mut self, node: Box<Node>) {
        match &mut self.neighbors {
            Some(neighbors) => {
                neighbors.push(node);
            }
            None => {
                self.neighbors = Some(vec![node]);
            }
        }
    }

    fn remove_neighbor(&mut self, value: &str) {
        match &mut self.neighbors {
            Some(neighbors) => {
                neighbors.retain(|node| node.get_value() != value);
            }
            None => (),
        }
    }

    fn get_neighbors(&self) -> Vec<Box<Node>> {
        match &self.neighbors {
            Some(neighbors) => neighbors.to_vec(),
            None => Vec::new(),
        }
    }

    fn print(&self) {
        println!("Node: {}", self.value);
        match &self.neighbors {
            Some(neighbors) => {
                for neighbor in neighbors {
                    println!("Neighbor: {}", neighbor.get_value());
                }
            }
            None => (),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            value: self.value.clone(),
            neighbors: match &self.neighbors {
                Some(neighbors) => Some(neighbors.clone()),
                None => None,
            },
        }
    }
}