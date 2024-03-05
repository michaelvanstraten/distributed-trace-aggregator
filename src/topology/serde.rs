use core::fmt;

use serde::{
    de::{MapAccess, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize,
};

use crate::topology::{VertexID, VertexWeight};

use super::{AdjacencyList, WeightedVertices};

impl Serialize for AdjacencyList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self)
    }
}

impl<'de> Deserialize<'de> for AdjacencyList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor {}

        impl<'de> Visitor<'de> for MapVisitor {
            type Value = AdjacencyList;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }

            #[cfg_attr(feature = "inline-more", inline)]
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut adjacency_list = AdjacencyList::default();

                while let Some((vertex_id, neighbouring_vertices)) = map.next_entry::<VertexID, WeightedVertices>()? {
                    adjacency_list.set_adjacent_vertices(vertex_id, neighbouring_vertices);
                }

                Ok(adjacency_list)
            }
        }

        let visitor = MapVisitor {};
        deserializer.deserialize_map(visitor)
    }
}

impl Serialize for WeightedVertices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_map(self)
    }
}

impl<'de> Deserialize<'de> for WeightedVertices {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor {}

        impl<'de> Visitor<'de> for MapVisitor {
            type Value = WeightedVertices;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a map")
            }

            #[cfg_attr(feature = "inline-more", inline)]
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut weighted_vertices = WeightedVertices::default();

                while let Some((vertex_id, vertex_weight)) =
                    map.next_entry::<VertexID, VertexWeight>()?
                {
                    weighted_vertices.set_vertex_weight(vertex_id, vertex_weight);
                }

                Ok(weighted_vertices)
            }
        }

        let visitor = MapVisitor {};
        deserializer.deserialize_map(visitor)
    }
}
