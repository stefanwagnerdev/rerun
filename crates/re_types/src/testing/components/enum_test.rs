// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/components/enum_test.fbs".

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

/// **Component**: A test of the enum type.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub enum EnumTest {
    /// Great film.
    Up = 1,

    /// Feeling blue.
    Down = 2,

    /// Correct.
    #[default]
    Right = 3,

    /// It's what's remaining.
    Left = 4,

    /// It's the only way to go.
    Forward = 5,

    /// Baby's got it.
    Back = 6,
}

impl ::re_types_core::reflection::Enum for EnumTest {
    #[inline]
    fn variants() -> &'static [Self] {
        &[
            Self::Up,
            Self::Down,
            Self::Right,
            Self::Left,
            Self::Forward,
            Self::Back,
        ]
    }

    #[inline]
    fn docstring_md(self) -> &'static str {
        match self {
            Self::Up => "Great film.",
            Self::Down => "Feeling blue.",
            Self::Right => "Correct.",
            Self::Left => "It's what's remaining.",
            Self::Forward => "It's the only way to go.",
            Self::Back => "Baby's got it.",
        }
    }
}

impl ::re_types_core::SizeBytes for EnumTest {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }

    #[inline]
    fn is_pod() -> bool {
        true
    }
}

impl std::fmt::Display for EnumTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "Down"),
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
            Self::Forward => write!(f, "Forward"),
            Self::Back => write!(f, "Back"),
        }
    }
}

::re_types_core::macros::impl_into_cow!(EnumTest);

impl ::re_types_core::Loggable for EnumTest {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.testing.components.EnumTest".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Union(
            std::sync::Arc::new(vec![
                Field::new("_null_markers", DataType::Null, true),
                Field::new("Up", DataType::Null, true),
                Field::new("Down", DataType::Null, true),
                Field::new("Right", DataType::Null, true),
                Field::new("Left", DataType::Null, true),
                Field::new("Forward", DataType::Null, true),
                Field::new("Back", DataType::Null, true),
            ]),
            Some(std::sync::Arc::new(vec![
                0i32, 1i32, 2i32, 3i32, 4i32, 5i32, 6i32,
            ])),
            UnionMode::Sparse,
        )
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
            // Sparse Arrow union
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            let num_variants = 6usize;
            let types = data
                .iter()
                .map(|a| match a.as_deref() {
                    None => 0,
                    Some(value) => *value as i8,
                })
                .collect();
            let fields: Vec<_> =
                std::iter::repeat(NullArray::new(DataType::Null, data.len()).boxed())
                    .take(1 + num_variants)
                    .collect();
            UnionArray::new(Self::arrow_datatype(), types, fields, None).boxed()
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
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::UnionArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.testing.components.EnumTest")?;
            let arrow_data_types = arrow_data.types();
            arrow_data_types
                .iter()
                .map(|typ| match typ {
                    0 => Ok(None),
                    1 => Ok(Some(Self::Up)),
                    2 => Ok(Some(Self::Down)),
                    3 => Ok(Some(Self::Right)),
                    4 => Ok(Some(Self::Left)),
                    5 => Ok(Some(Self::Forward)),
                    6 => Ok(Some(Self::Back)),
                    _ => Err(DeserializationError::missing_union_arm(
                        Self::arrow_datatype(),
                        "<invalid>",
                        *typ as _,
                    )),
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.components.EnumTest")?
        })
    }
}
