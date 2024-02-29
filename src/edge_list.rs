use crate::graph::Graph;

pub struct EdgeList {
    // from, to, weight
    edges: Vec<(i32, i32, i32)>,
    is_directed: bool,
    is_weighted: bool,
}


impl Graph for EdgeList {
    fn new(is_directed: bool, is_weighted: bool) -> Self {
        EdgeList {
            edges: Vec::new(),
            is_directed,
            is_weighted,
        }
    }
    
    fn add_node(&mut self, value: i32) {
        print!("{}", value.to_string());
        // Do nothing
    }
    
    fn add_edge(&mut self, from: i32, to: i32, weight: i32) {
        let mut weight = weight;
        match !self.is_weighted {
            true => {
                weight = 1;
            }
            false => (),
        }
        
        if !self.is_directed {
            self.edges.push((to, from, weight));
            self.edges.push((from, to, weight));
            return;
        }
        self.edges.push((from, to, weight));
    }

    fn get_adjacent(&self, value: i32) -> Vec<i32> {
        let mut adjacent = Vec::new();
        for (from, to, _) in &self.edges {
            if *from == value {
                adjacent.push(*to);
            }
        }
        adjacent
    }

    fn get_node(&self, value: i32) -> i32 {
        value
    }

    fn is_neighbor(&self, from: i32, to: i32) -> bool {
        for (f, t, _) in &self.edges {
            if *f == from && *t == to {
                return true;
            }
        }
        false
    }

    fn print(&self) {
        for (from, to, weight) in &self.edges {
            println!("{} -> {} ({})", from, to, weight);
        }
    }
} 
