use std::{hash::RandomState, mem};

use hashbrown::raw::RawTable;

use crate::hashbrown_utils::{equivalent_key, make_hasher};

use super::{VertexID, VertexWeight};

pub struct WeightedVertices {
    hash_builder: RandomState,
    vertices: RawTable<(VertexID, VertexWeight)>,
}

impl WeightedVertices {
    pub fn new() -> Self {
        WeightedVertices {
            vertices: RawTable::new(),
            hash_builder: RandomState::new(),
        }
    }

    pub fn set_vertex_weight(
        &mut self,
        vertex_id: VertexID,
        vertex_weight: VertexWeight,
    ) -> Option<(VertexID, VertexWeight)> {
        match self.vertices.find_or_find_insert_slot(
            vertex_id,
            equivalent_key(&vertex_id),
            make_hasher(&self.hash_builder),
        ) {
            Ok(bucket) => Some((
                vertex_id,
                mem::replace(unsafe { &mut bucket.as_mut().1 }, vertex_weight),
            )),
            Err(slot) => {
                unsafe {
                    self.vertices
                        .insert_in_slot(vertex_id, slot, (vertex_id, vertex_weight));
                }
                None
            }
        }
    }

    pub fn get_vertex_weigth(&self, vertex_id: VertexID) -> Option<&VertexWeight> {
        self.vertices
            .get(vertex_id, equivalent_key(&vertex_id))
            .map(|(_, vertex_weight)| vertex_weight)
    }

    pub fn get_vertex_weigth_mut(&mut self, vertex_id: VertexID) -> Option<&mut VertexWeight> {
        self.vertices
            .get_mut(vertex_id, equivalent_key(&vertex_id))
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
