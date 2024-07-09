// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/resolution.fbs".

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

/// **Component**: Pixel resolution width & height, e.g. of a camera sensor.
///
/// Typically in integer units, but for some use cases floating point may be used.
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Resolution(pub crate::datatypes::Vec2D);

impl ::re_types_core::SizeBytes for Resolution {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Vec2D>::is_pod()
    }
}

impl<T: Into<crate::datatypes::Vec2D>> From<T> for Resolution {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Vec2D> for Resolution {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Vec2D {
        &self.0
    }
}

impl std::ops::Deref for Resolution {
    type Target = crate::datatypes::Vec2D;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Vec2D {
        &self.0
    }
}

impl std::ops::DerefMut for Resolution {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Vec2D {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(Resolution);

impl ::re_types_core::Loggable for Resolution {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.Resolution".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        crate::datatypes::Vec2D::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Vec2D::to_arrow_opt(data.into_iter().map(|datum| {
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
        crate::datatypes::Vec2D::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow2::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        crate::datatypes::Vec2D::from_arrow(arrow_data).map(|v| v.into_iter().map(Self).collect())
    }
}
