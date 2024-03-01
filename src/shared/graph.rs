// > Module: graph
pub trait TGraph {
    fn new(is_directed: bool, is_weighted: bool) -> Self;
    fn add_node(&mut self, label: &str);
    fn add_edge(&mut self, from: &str, to: &str, weight: i32);
    fn remove_node(&mut self, label: &str);
    fn remove_edge(&mut self, from: &str, to: &str);
    fn is_neighbor(&self, from: &str, to: &str) -> bool;
    fn get_neighbors(&self, label: &str) -> Vec<&str>;
    fn print(&self);
}

pub trait TNode {
    fn new(value: &str) -> Self;
    fn new_with_neighbors(value: &str, neighbors: Vec<Box<Self>>) -> Self;
    fn get_value(&self) -> &str;
    fn set_value(&mut self, value: &str);
    fn add_neighbor(&mut self, node: Box<Self>);
    fn remove_neighbor(&mut self, id: &str);
    fn get_neighbors(&self) -> Vec<Box<Self>>;
    fn print(&self);
}
