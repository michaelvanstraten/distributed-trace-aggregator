use std::hash::Hash;

use crate::{hashbrown_utils, topology};
use hashbrown::{hash_map::DefaultHashBuilder, HashSet};
use hashbrown_utils::make_hash;
use topology::AdjacencyList;

pub struct TraceGraph<T, H = DefaultHashBuilder> {
    topology: AdjacencyList,
    nodes: HashSet<T>,
    hash_builder: H,
}

impl<T> TraceGraph<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> TraceGraph<T>
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

impl<T> Default for TraceGraph<T> {
    fn default() -> Self {
        Self {
            topology: AdjacencyList::default(),
            nodes: HashSet::default(),
            hash_builder: DefaultHashBuilder::default(),
        }
    }
}
