#![allow(unused)]

use hashbrown::{raw::RawTable, hash_map::DefaultHashBuilder};
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
        Self::default()
    }
}

impl<Node> Graph<Node>
where
    Node: Hash,
{
    pub fn add_node_relation(&mut self, from: Node, to: Node) {}
}

impl<Node> Default for Graph<Node> {
    fn default() -> Self {
        Self {
            _topology: AdjacencyList::default(),
            nodes: RawTable::default(),
            hash_builder: DefaultHashBuilder::default()
        }
    }
}
