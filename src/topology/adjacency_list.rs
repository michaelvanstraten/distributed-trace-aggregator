use std::hash::BuildHasherDefault;

use hashbrown::{hash_map::DefaultHashBuilder, hash_table, raw::RawTable, HashTable};

use crate::hashbrown_utils::{equivalent_key, make_hasher};

use super::{VertexID, WeightedVertices};

#[derive(Default)]
pub struct AdjacencyList<H = DefaultHashBuilder> {
    vertices: HashTable<(VertexID, WeightedVertices)>,
    hash_builder: H,
}

impl AdjacencyList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_edge(&mut self, from: VertexID, to: VertexID) {
        match self
            .vertices
            .entry(from, equivalent_key(&from), make_hasher(&self.hash_builder))
        {
            hash_table::Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().1.increment_vertex_weight(to, 1)
            }
            hash_table::Entry::Vacant(vacant_entry) => {
                let mut neighbouring_vertices = WeightedVertices::new();
                neighbouring_vertices.set_vertex_weight(to, 1);

                vacant_entry.insert((from, neighbouring_vertices));
            }
        }
    }
}
    }
}
