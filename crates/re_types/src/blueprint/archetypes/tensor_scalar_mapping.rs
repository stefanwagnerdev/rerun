// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/tensor_scalar_mapping.fbs".

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
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: Configures how tensor scalars are mapped to color.
#[derive(Clone, Debug, Default)]
pub struct TensorScalarMapping {
    /// Filter used when zooming in on the tensor.
    ///
    /// Note that the filter is applied to the scalar values *before* they are mapped to color.
    pub mag_filter: Option<crate::components::MagnificationFilter>,

    /// How scalar values map to colors.
    pub colormap: Option<crate::components::Colormap>,

    /// Gamma exponent applied to normalized values before mapping to color.
    ///
    /// Raises the normalized values to the power of this value before mapping to color.
    /// Acts like an inverse brightness. Defaults to 1.0.
    pub gamma: Option<crate::components::GammaCorrection>,
}

impl ::re_types_core::SizeBytes for TensorScalarMapping {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.mag_filter.heap_size_bytes()
            + self.colormap.heap_size_bytes()
            + self.gamma.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::components::MagnificationFilter>>::is_pod()
            && <Option<crate::components::Colormap>>::is_pod()
            && <Option<crate::components::GammaCorrection>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        ["rerun.blueprint.components.TensorScalarMappingIndicator".into()]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.MagnificationFilter".into(),
            "rerun.components.Colormap".into(),
            "rerun.components.GammaCorrection".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.TensorScalarMappingIndicator".into(),
            "rerun.components.MagnificationFilter".into(),
            "rerun.components.Colormap".into(),
            "rerun.components.GammaCorrection".into(),
        ]
    });

impl TensorScalarMapping {
    /// The total number of components in the archetype: 0 required, 1 recommended, 3 optional
    pub const NUM_COMPONENTS: usize = 4usize;
}

/// Indicator component for the [`TensorScalarMapping`] [`::re_types_core::Archetype`]
pub type TensorScalarMappingIndicator =
    ::re_types_core::GenericIndicatorComponent<TensorScalarMapping>;

impl ::re_types_core::Archetype for TensorScalarMapping {
    type Indicator = TensorScalarMappingIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.TensorScalarMapping".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Tensor scalar mapping"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: TensorScalarMappingIndicator = TensorScalarMappingIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let mag_filter =
            if let Some(array) = arrays_by_name.get("rerun.components.MagnificationFilter") {
                <crate::components::MagnificationFilter>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.TensorScalarMapping#mag_filter")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let colormap = if let Some(array) = arrays_by_name.get("rerun.components.Colormap") {
            <crate::components::Colormap>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.TensorScalarMapping#colormap")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let gamma = if let Some(array) = arrays_by_name.get("rerun.components.GammaCorrection") {
            <crate::components::GammaCorrection>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.TensorScalarMapping#gamma")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self {
            mag_filter,
            colormap,
            gamma,
        })
    }
}

impl ::re_types_core::AsComponents for TensorScalarMapping {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.mag_filter
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.colormap
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.gamma
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for TensorScalarMapping {}

impl TensorScalarMapping {
    /// Create a new `TensorScalarMapping`.
    #[inline]
    pub fn new() -> Self {
        Self {
            mag_filter: None,
            colormap: None,
            gamma: None,
        }
    }

    /// Filter used when zooming in on the tensor.
    ///
    /// Note that the filter is applied to the scalar values *before* they are mapped to color.
    #[inline]
    pub fn with_mag_filter(
        mut self,
        mag_filter: impl Into<crate::components::MagnificationFilter>,
    ) -> Self {
        self.mag_filter = Some(mag_filter.into());
        self
    }

    /// How scalar values map to colors.
    #[inline]
    pub fn with_colormap(mut self, colormap: impl Into<crate::components::Colormap>) -> Self {
        self.colormap = Some(colormap.into());
        self
    }

    /// Gamma exponent applied to normalized values before mapping to color.
    ///
    /// Raises the normalized values to the power of this value before mapping to color.
    /// Acts like an inverse brightness. Defaults to 1.0.
    #[inline]
    pub fn with_gamma(mut self, gamma: impl Into<crate::components::GammaCorrection>) -> Self {
        self.gamma = Some(gamma.into());
        self
    }
}
