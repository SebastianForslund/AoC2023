use std::rc::Rc;

pub struct Edge<T> {
    pub dest: Rc<Node<T>>,
    pub weight: i32,
}

pub struct Node<T> {
    pub edges: Vec<Rc<Edge<T>>>,
    pub value: T,
}

pub struct Graph<T> {
    pub start: Rc<Node<T>>,
    pub nodes: Vec<Node<T>>,
}

impl<T> Graph<T> {
    pub fn new(origin_node_value: T) -> Graph<T> {
        let origin = Node::<T> {
            edges: Vec::new(),
            value: origin_node_value,
        };
        return Graph {
            start: Rc::new(origin),
            nodes: Vec::new(),
        };
    }
}
