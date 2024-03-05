#![allow(unused)]

use hashbrown::{hash_map::DefaultHashBuilder, raw::RawTable, HashSet};
use hashbrown_utils::{make_hash, make_hasher};
use std::hash::Hash;
use topology::{AdjacencyList, VertexID};

mod hashbrown_utils;
mod node;
mod topology;

pub struct Graph<T, H = DefaultHashBuilder> {
    topology: AdjacencyList,
    nodes: HashSet<T>,
    hash_builder: H,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Graph<T>
where
    T: Hash + Eq,
{
    pub fn add_node_relation(&mut self, from: T, to: T) {
        self.topology.insert_edge(
            make_hash(&self.hash_builder, &from),
            make_hash(&self.hash_builder, &to),
        );

        self.nodes.insert(from);
        self.nodes.insert(to);
    }
}

impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self {
            topology: AdjacencyList::default(),
            nodes: HashSet::default(),
            hash_builder: DefaultHashBuilder::default(),
        }
    }
}
