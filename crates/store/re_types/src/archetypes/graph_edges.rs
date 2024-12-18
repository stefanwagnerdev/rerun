// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/graph_edges.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A list of edges in a graph.
///
/// By default, edges are undirected.
///
/// ⚠️ **This type is experimental and may be removed in future versions**
///
/// ## Example
///
/// ### Simple directed graph
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_graph_directed").spawn()?;
///
///     rec.log(
///         "simple",
///         &[
///             &rerun::GraphNodes::new(["a", "b", "c"])
///                 .with_positions([(0.0, 100.0), (-100.0, 0.0), (100.0, 0.0)])
///                 .with_labels(["A", "B", "C"]) as &dyn rerun::AsComponents,
///             &rerun::GraphEdges::new([("a", "b"), ("b", "c"), ("c", "a")]).with_directed_edges(),
///         ],
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/1200w.png">
///   <img src="https://static.rerun.io/graph_directed/ca29a37b65e1e0b6482251dce401982a0bc568fa/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphEdges {
    /// A list of node tuples.
    pub edges: Vec<crate::components::GraphEdge>,

    /// Specifies if the graph is directed or undirected.
    ///
    /// If no [`components::GraphType`][crate::components::GraphType] is provided, the graph is assumed to be undirected.
    pub graph_type: Option<crate::components::GraphType>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GraphEdges".into()),
            component_name: "rerun.components.GraphEdge".into(),
            archetype_field_name: Some("edges".into()),
        }]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphEdges".into()),
                component_name: "rerun.components.GraphType".into(),
                archetype_field_name: Some("graph_type".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphEdges".into()),
                component_name: "rerun.components.GraphEdgesIndicator".into(),
                archetype_field_name: None,
            },
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphEdges".into()),
                component_name: "rerun.components.GraphEdge".into(),
                archetype_field_name: Some("edges".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphEdges".into()),
                component_name: "rerun.components.GraphType".into(),
                archetype_field_name: Some("graph_type".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.GraphEdges".into()),
                component_name: "rerun.components.GraphEdgesIndicator".into(),
                archetype_field_name: None,
            },
        ]
    });

impl GraphEdges {
    /// The total number of components in the archetype: 1 required, 2 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`GraphEdges`] [`::re_types_core::Archetype`]
pub type GraphEdgesIndicator = ::re_types_core::GenericIndicatorComponent<GraphEdges>;

impl ::re_types_core::Archetype for GraphEdges {
    type Indicator = GraphEdgesIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.GraphEdges".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Graph edges"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: GraphEdgesIndicator = GraphEdgesIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow2_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let edges = {
            let array = arrays_by_name
                .get("rerun.components.GraphEdge")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.GraphEdges#edges")?;
            <crate::components::GraphEdge>::from_arrow2_opt(&**array)
                .with_context("rerun.archetypes.GraphEdges#edges")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.GraphEdges#edges")?
        };
        let graph_type = if let Some(array) = arrays_by_name.get("rerun.components.GraphType") {
            <crate::components::GraphType>::from_arrow2_opt(&**array)
                .with_context("rerun.archetypes.GraphEdges#graph_type")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { edges, graph_type })
    }
}

impl ::re_types_core::AsComponents for GraphEdges {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.edges as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.GraphEdges".into()),
                        archetype_field_name: Some(("edges").into()),
                        component_name: ("rerun.components.GraphEdge").into(),
                    }),
                }
            }),
            (self
                .graph_type
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.archetypes.GraphEdges".into()),
                    archetype_field_name: Some(("graph_type").into()),
                    component_name: ("rerun.components.GraphType").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for GraphEdges {}

impl GraphEdges {
    /// Create a new `GraphEdges`.
    #[inline]
    pub fn new(edges: impl IntoIterator<Item = impl Into<crate::components::GraphEdge>>) -> Self {
        Self {
            edges: edges.into_iter().map(Into::into).collect(),
            graph_type: None,
        }
    }

    /// Specifies if the graph is directed or undirected.
    ///
    /// If no [`components::GraphType`][crate::components::GraphType] is provided, the graph is assumed to be undirected.
    #[inline]
    pub fn with_graph_type(mut self, graph_type: impl Into<crate::components::GraphType>) -> Self {
        self.graph_type = Some(graph_type.into());
        self
    }
}

impl ::re_types_core::SizeBytes for GraphEdges {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.edges.heap_size_bytes() + self.graph_type.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::GraphEdge>>::is_pod()
            && <Option<crate::components::GraphType>>::is_pod()
    }
}
