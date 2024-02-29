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
    
    fn remove_node(&mut self, value: i32) {
        self.edges.retain(|(from, to, _)| *from != value && *to != value);
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

    fn remove_edge(&mut self, from: i32, to: i32) {
        let mut index = 0;
        for (i, (f, t, _)) in self.edges.iter().enumerate() {
            if *f == from && *t == to {
                index = i;
                break;
            }
        }
        self.edges.remove(index);
    }

    fn get_neighbors(&self, value: i32) -> Vec<i32> {
        let mut adjacent = Vec::new();
        for (from, to, _) in &self.edges {
            if *from == value {
                adjacent.push(*to);
            }
        }
        adjacent
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
