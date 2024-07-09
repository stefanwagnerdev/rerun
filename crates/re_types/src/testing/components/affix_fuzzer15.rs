// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AffixFuzzer15(pub Option<crate::testing::datatypes::AffixFuzzer3>);

impl ::re_types_core::SizeBytes for AffixFuzzer15 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::testing::datatypes::AffixFuzzer3>>::is_pod()
    }
}

impl<T: Into<Option<crate::testing::datatypes::AffixFuzzer3>>> From<T> for AffixFuzzer15 {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<Option<crate::testing::datatypes::AffixFuzzer3>> for AffixFuzzer15 {
    #[inline]
    fn borrow(&self) -> &Option<crate::testing::datatypes::AffixFuzzer3> {
        &self.0
    }
}

impl std::ops::Deref for AffixFuzzer15 {
    type Target = Option<crate::testing::datatypes::AffixFuzzer3>;

    #[inline]
    fn deref(&self) -> &Option<crate::testing::datatypes::AffixFuzzer3> {
        &self.0
    }
}

impl std::ops::DerefMut for AffixFuzzer15 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Option<crate::testing::datatypes::AffixFuzzer3> {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(AffixFuzzer15);

impl ::re_types_core::Loggable for AffixFuzzer15 {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.testing.components.AffixFuzzer15".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Union(
            std::sync::Arc::new(vec![
                Field::new("_null_markers", DataType::Null, true),
                Field::new("degrees", DataType::Float32, false),
                Field::new(
                    "craziness",
                    DataType::List(std::sync::Arc::new(Field::new(
                        "item",
                        <crate::testing::datatypes::AffixFuzzer1>::arrow_datatype(),
                        false,
                    ))),
                    false,
                ),
                Field::new(
                    "fixed_size_shenanigans",
                    DataType::FixedSizeList(
                        std::sync::Arc::new(Field::new("item", DataType::Float32, false)),
                        3usize,
                    ),
                    false,
                ),
                Field::new("empty_variant", DataType::Null, true),
            ]),
            Some(std::sync::Arc::new(vec![0i32, 1i32, 2i32, 3i32, 4i32])),
            UnionMode::Dense,
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
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().0).flatten();
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                _ = data0_bitmap;
                crate::testing::datatypes::AffixFuzzer3::to_arrow_opt(data0)?
            }
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
        Ok(
            crate::testing::datatypes::AffixFuzzer3::from_arrow_opt(arrow_data)
                .with_context("rerun.testing.components.AffixFuzzer15#single_optional_union")?
                .into_iter()
                .map(Ok)
                .map(|res| res.map(|v| Some(Self(v))))
                .collect::<DeserializationResult<Vec<Option<_>>>>()
                .with_context("rerun.testing.components.AffixFuzzer15#single_optional_union")
                .with_context("rerun.testing.components.AffixFuzzer15")?,
        )
    }
}
