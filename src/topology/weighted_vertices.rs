use std::{hash::RandomState, mem};

use hashbrown::{hash_map::DefaultHashBuilder, hash_table, raw::RawTable, HashTable};

use crate::hashbrown_utils::{equivalent_key, make_hasher};

use super::{VertexID, VertexWeight};

#[derive(Default)]
pub struct WeightedVertices<H = DefaultHashBuilder> {
    hash_builder: H,
    vertices: HashTable<(VertexID, VertexWeight)>,
}

impl WeightedVertices {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_vertex_weight(
        &mut self,
        vertex_id: VertexID,
        vertex_weight: VertexWeight,
    ) -> Option<VertexWeight> {
        match self.vertices.entry(
            vertex_id,
            equivalent_key(&vertex_id),
            make_hasher(&self.hash_builder),
        ) {
            hash_table::Entry::Occupied(mut occupied_entry) => {
                Some(mem::replace(&mut occupied_entry.get_mut().1, vertex_weight))
            }
            hash_table::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert((vertex_id, vertex_weight));
                None
            }
        }
    }

    pub fn get_vertex_weigth(&self, vertex_id: VertexID) -> Option<&VertexWeight> {
        self.vertices
            .find(vertex_id, equivalent_key(&vertex_id))
            .map(|(_, vertex_weight)| vertex_weight)
    }

    pub fn get_vertex_weigth_mut(&mut self, vertex_id: VertexID) -> Option<&mut VertexWeight> {
        self.vertices
            .find_mut(vertex_id, equivalent_key(&vertex_id))
            .map(|(_, vertex_weight)| vertex_weight)
    }

    pub fn increment_vertex_weight(&mut self, vertex_id: VertexID, by: VertexWeight) {
        if let Some(vertex_weight) = self.get_vertex_weigth_mut(vertex_id) {
            *vertex_weight += by;
        } else {
            self.set_vertex_weight(vertex_id, by);
        }
    }
}
