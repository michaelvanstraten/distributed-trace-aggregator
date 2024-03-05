use std::mem;

use hashbrown::{
    hash_map::DefaultHashBuilder,
    hash_table::{self, Iter},
    HashTable,
};

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

    pub fn set_adjacent_vertices(
        &mut self,
        vertex_id: VertexID,
        neighbouring_vertices: WeightedVertices,
    ) -> Option<WeightedVertices> {
        match self.vertices.entry(
            vertex_id,
            equivalent_key(&vertex_id),
            make_hasher(&self.hash_builder),
        ) {
            hash_table::Entry::Occupied(mut occupied_entry) => Some(mem::replace(
                &mut occupied_entry.get_mut().1,
                neighbouring_vertices,
            )),
            hash_table::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert((vertex_id, neighbouring_vertices));
                None
            }
        }
    }
}

impl<'a> IntoIterator for &'a AdjacencyList {
    type Item = (&'a VertexID, &'a WeightedVertices);
    type IntoIter = std::iter::Map<
        Iter<'a, (VertexID, WeightedVertices)>,
        fn(&(VertexID, WeightedVertices)) -> (&VertexID, &WeightedVertices),
    >;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> Self::IntoIter {
        self.vertices.iter().map(|item| (&item.0, &item.1))
    }
}
