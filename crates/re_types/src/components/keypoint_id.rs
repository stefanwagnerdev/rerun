// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/keypoint_id.fbs".

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

/// **Component**: A 16-bit ID representing a type of semantic keypoint within a class.
///
/// `KeypointId`s are only meaningful within the context of a [`crate::datatypes::ClassDescription`].
///
/// Used to look up an [`crate::datatypes::AnnotationInfo`] for a Keypoint within the [`crate::components::AnnotationContext`].
#[derive(
    Clone,
    Debug,
    Default,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    bytemuck::Pod,
    bytemuck::Zeroable,
)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct KeypointId(pub crate::datatypes::KeypointId);

impl ::re_types_core::SizeBytes for KeypointId {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::KeypointId>::is_pod()
    }
}

impl<T: Into<crate::datatypes::KeypointId>> From<T> for KeypointId {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::KeypointId> for KeypointId {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::KeypointId {
        &self.0
    }
}

impl std::ops::Deref for KeypointId {
    type Target = crate::datatypes::KeypointId;

    #[inline]
    fn deref(&self) -> &crate::datatypes::KeypointId {
        &self.0
    }
}

impl std::ops::DerefMut for KeypointId {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::KeypointId {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(KeypointId);

impl ::re_types_core::Loggable for KeypointId {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.KeypointId".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        crate::datatypes::KeypointId::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::KeypointId::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::KeypointId::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow2::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::KeypointId::from_arrow(arrow_data).map(bytemuck::cast_vec)
    }
}
