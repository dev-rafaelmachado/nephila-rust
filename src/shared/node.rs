use crate::shared::graph::TNode;
pub struct Node {
    value: String,
    neighbors: Option<Vec<(String, i32)>>,
}

impl TNode for Node {
    fn new(value: &str) -> Self {
        Node {
            value: value.to_string(),
            neighbors: None,
        }
    }

    fn new_with_neighbors(value: &str, neighbors: Vec<(String, i32)>) -> Self {
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

    fn add_neighbor(&mut self, node: &str, weight: i32) {
        match &mut self.neighbors {
            Some(neighbors) => {
                neighbors.push((node.to_string(), weight));
            }
            None => {
                self.neighbors = Some(vec![(node.to_string(), weight)]);
            }
        }
    }

    fn remove_neighbor(&mut self, value: &str) {
        match &mut self.neighbors {
            Some(neighbors) => {
                neighbors.retain(|(node, _)| *node != value);
            }
            None => (),
        }
    }

    fn is_neighbor(&mut self, to: &str) -> bool {
        self.neighbors.as_ref().map_or(false, |neighbors| {
            neighbors.iter().any(|(node, _)| *node == to)
        })
    }

    fn get_neighbors(&self) -> Vec<(&str, i32)> {
        match &self.neighbors {
            Some(neighbors) => neighbors.iter().map(|(node, weight)| (node.as_str(), *weight)).collect(),
            None => Vec::new(),
        }
    }

    fn print(&self) {
        //X: [(Y, 1), (Z, 2)]
        let neighbors = match &self.neighbors {
            Some(neighbors) => neighbors.iter().map(|(node, weight)| format!("({},{})", node, weight)).collect::<Vec<String>>().join(", "),
            None => String::from(""),
        };
        println!("{}: [{}]", self.value, neighbors);
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