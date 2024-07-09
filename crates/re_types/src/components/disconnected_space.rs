// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/disconnected_space.fbs".

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

/// **Component**: Spatially disconnect this entity from its parent.
///
/// Specifies that the entity path at which this is logged is spatially disconnected from its parent,
/// making it impossible to transform the entity path into its parent's space and vice versa.
/// It *only* applies to space views that work with spatial transformations, i.e. 2D & 3D space views.
/// This is useful for specifying that a subgraph is independent of the rest of the scene.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct DisconnectedSpace(
    /// Whether the entity path at which this is logged is disconnected from its parent.
    ///
    /// Set to true to disconnect the entity from its parent.
    /// Set to false to disable the effects of this component, (re-)connecting the entity to its parent again.
    pub bool,
);

impl ::re_types_core::SizeBytes for DisconnectedSpace {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <bool>::is_pod()
    }
}

impl From<bool> for DisconnectedSpace {
    #[inline]
    fn from(is_disconnected: bool) -> Self {
        Self(is_disconnected)
    }
}

impl From<DisconnectedSpace> for bool {
    #[inline]
    fn from(value: DisconnectedSpace) -> Self {
        value.0
    }
}

impl std::ops::Deref for DisconnectedSpace {
    type Target = bool;

    #[inline]
    fn deref(&self) -> &bool {
        &self.0
    }
}

impl std::ops::DerefMut for DisconnectedSpace {
    #[inline]
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(DisconnectedSpace);

impl ::re_types_core::Loggable for DisconnectedSpace {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.DisconnectedSpace".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Boolean
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().0);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            BooleanArray::new(
                Self::arrow_datatype(),
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
            )
            .boxed()
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<BooleanArray>()
            .ok_or_else(|| {
                let expected = Self::arrow_datatype();
                let actual = arrow_data.data_type().clone();
                DeserializationError::datatype_mismatch(expected, actual)
            })
            .with_context("rerun.components.DisconnectedSpace#is_disconnected")?
            .into_iter()
            .map(|v| v.ok_or_else(DeserializationError::missing_data))
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.DisconnectedSpace#is_disconnected")
            .with_context("rerun.components.DisconnectedSpace")?)
    }
}
