use hashbrown::raw::RawTable;
use std::hash::Hash;
use topology::{AdjacencyList, VertexID};

mod hashbrown_utils;
mod node;
mod topology;

pub struct Graph<Node> {
    _topology: AdjacencyList,
    nodes: RawTable<(VertexID, Node)>,
}

impl<Node> Graph<Node> {
    pub fn new() -> Self {
        Graph {
            _topology: AdjacencyList::new(),
            nodes: RawTable::new(),
        }
    }
}

impl<Node> Graph<Node>
where
    Node: Hash,
{
    pub fn add_node_relation(&mut self, from: Node, to: Node) {}
}
