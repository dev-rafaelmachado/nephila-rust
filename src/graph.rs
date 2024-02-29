pub trait Graph {
    fn new(is_directed: bool, is_weighted: bool) -> Self;
    fn add_node(&mut self, value: i32);
    fn add_edge(&mut self, from: i32, to: i32, weight: i32);
    fn remove_node(&mut self, value: i32);
    fn remove_edge(&mut self, from: i32, to: i32);
    fn get_neighbors(&self, value: i32) -> Vec<i32>;
    fn is_neighbor(&self, from: i32, to: i32) -> bool;
    fn print(&self);
}

// # methods
// - add_node
// - add_edge
// - remove_node
// - remove_edge
// - get_neighbors
// - is_neighbor
// - print

