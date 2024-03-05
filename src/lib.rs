use hashbrown::{hash_map::DefaultHashBuilder, raw::RawTable};
use std::hash::Hash;
use topology::{AdjacencyList, VertexID};

mod hashbrown_utils;
mod node;
mod topology;

pub struct Graph<Node, H = DefaultHashBuilder> {
    _topology: AdjacencyList,
    nodes: RawTable<(VertexID, Node)>,
    hash_builder: H,
}

impl<Node> Graph<Node> {
    pub fn new() -> Self {
        Graph {
            _topology: AdjacencyList::new(),
            nodes: RawTable::new(),
            hash_builder: DefaultHashBuilder::default(),
        }
    }
}

impl<Node> Graph<Node>
where
    Node: Hash,
{
    pub fn add_node_relation(&mut self, from: Node, to: Node) {}
}
