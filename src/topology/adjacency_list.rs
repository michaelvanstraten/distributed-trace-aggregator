use std::hash::BuildHasherDefault;

use hashbrown::{hash_map::DefaultHashBuilder, raw::RawTable};

use crate::hashbrown_utils::{equivalent_key, make_hasher};

use super::{VertexID, WeightedVertices};

#[derive(Default)]
pub struct AdjacencyList<H = DefaultHashBuilder> {
    vertices: RawTable<(VertexID, WeightedVertices)>,
    hash_builder: H,
}

impl AdjacencyList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_edge(&mut self, from: VertexID, to: VertexID) {
        match self.vertices.find_or_find_insert_slot(
            from,
            equivalent_key(&from),
            make_hasher(&self.hash_builder),
        ) {
            Ok(bucket) => unsafe { bucket.as_mut() }.1.increment_vertex_weight(to, 1),
            Err(slot) => {
                let mut neighbouring_vertices = WeightedVertices::new();
                neighbouring_vertices.set_vertex_weight(to, 1);

                unsafe {
                    self.vertices
                        .insert_in_slot(from, slot, (from, neighbouring_vertices))
                };
            }
        };
    }
}
