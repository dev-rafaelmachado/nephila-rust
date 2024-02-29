pub trait Graph {
    fn new(is_directed: bool, is_weighted: bool) -> Self;
    fn add_node(&mut self, value: i32);
    fn add_edge(&mut self, from: i32, to: i32, weight: i32);
    fn get_adjacent(&self, value: i32) -> Vec<i32>;
    fn get_node(&self, value: i32) -> i32;
    fn is_neighbor(&self, from: i32, to: i32) -> bool;
    fn print(&self);
}

