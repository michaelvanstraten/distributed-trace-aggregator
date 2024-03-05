#![allow(unused)]

use hashbrown::{hash_map::DefaultHashBuilder, raw::RawTable, HashSet};
use hashbrown_utils::{make_hash, make_hasher};
use std::hash::Hash;
use topology::{AdjacencyList, VertexID};

mod hashbrown_utils;
mod node;
mod topology;

pub struct Graph<Node, H = DefaultHashBuilder> {
    topology: AdjacencyList,
    nodes: HashSet<Node>,
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
            topology: AdjacencyList::default(),
            nodes: HashSet::default(),
            hash_builder: DefaultHashBuilder::default(),
        }
    }
}
