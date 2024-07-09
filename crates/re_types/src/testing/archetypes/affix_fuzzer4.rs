// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

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

#[derive(Clone, Debug, PartialEq)]
pub struct AffixFuzzer4 {
    pub fuzz2101: Option<Vec<crate::testing::components::AffixFuzzer1>>,
    pub fuzz2102: Option<Vec<crate::testing::components::AffixFuzzer2>>,
    pub fuzz2103: Option<Vec<crate::testing::components::AffixFuzzer3>>,
    pub fuzz2104: Option<Vec<crate::testing::components::AffixFuzzer4>>,
    pub fuzz2105: Option<Vec<crate::testing::components::AffixFuzzer5>>,
    pub fuzz2106: Option<Vec<crate::testing::components::AffixFuzzer6>>,
    pub fuzz2107: Option<Vec<crate::testing::components::AffixFuzzer7>>,
    pub fuzz2108: Option<Vec<crate::testing::components::AffixFuzzer8>>,
    pub fuzz2109: Option<Vec<crate::testing::components::AffixFuzzer9>>,
    pub fuzz2110: Option<Vec<crate::testing::components::AffixFuzzer10>>,
    pub fuzz2111: Option<Vec<crate::testing::components::AffixFuzzer11>>,
    pub fuzz2112: Option<Vec<crate::testing::components::AffixFuzzer12>>,
    pub fuzz2113: Option<Vec<crate::testing::components::AffixFuzzer13>>,
    pub fuzz2114: Option<Vec<crate::testing::components::AffixFuzzer14>>,
    pub fuzz2115: Option<Vec<crate::testing::components::AffixFuzzer15>>,
    pub fuzz2116: Option<Vec<crate::testing::components::AffixFuzzer16>>,
    pub fuzz2117: Option<Vec<crate::testing::components::AffixFuzzer17>>,
    pub fuzz2118: Option<Vec<crate::testing::components::AffixFuzzer18>>,
}

impl ::re_types_core::SizeBytes for AffixFuzzer4 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.fuzz2101.heap_size_bytes()
            + self.fuzz2102.heap_size_bytes()
            + self.fuzz2103.heap_size_bytes()
            + self.fuzz2104.heap_size_bytes()
            + self.fuzz2105.heap_size_bytes()
            + self.fuzz2106.heap_size_bytes()
            + self.fuzz2107.heap_size_bytes()
            + self.fuzz2108.heap_size_bytes()
            + self.fuzz2109.heap_size_bytes()
            + self.fuzz2110.heap_size_bytes()
            + self.fuzz2111.heap_size_bytes()
            + self.fuzz2112.heap_size_bytes()
            + self.fuzz2113.heap_size_bytes()
            + self.fuzz2114.heap_size_bytes()
            + self.fuzz2115.heap_size_bytes()
            + self.fuzz2116.heap_size_bytes()
            + self.fuzz2117.heap_size_bytes()
            + self.fuzz2118.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<Vec<crate::testing::components::AffixFuzzer1>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer2>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer3>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer4>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer5>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer6>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer7>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer8>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer9>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer10>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer11>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer12>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer13>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer14>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer15>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer16>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer17>>>::is_pod()
            && <Option<Vec<crate::testing::components::AffixFuzzer18>>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.testing.components.AffixFuzzer4Indicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 18usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.testing.components.AffixFuzzer1".into(),
            "rerun.testing.components.AffixFuzzer2".into(),
            "rerun.testing.components.AffixFuzzer3".into(),
            "rerun.testing.components.AffixFuzzer4".into(),
            "rerun.testing.components.AffixFuzzer5".into(),
            "rerun.testing.components.AffixFuzzer6".into(),
            "rerun.testing.components.AffixFuzzer7".into(),
            "rerun.testing.components.AffixFuzzer8".into(),
            "rerun.testing.components.AffixFuzzer9".into(),
            "rerun.testing.components.AffixFuzzer10".into(),
            "rerun.testing.components.AffixFuzzer11".into(),
            "rerun.testing.components.AffixFuzzer12".into(),
            "rerun.testing.components.AffixFuzzer13".into(),
            "rerun.testing.components.AffixFuzzer14".into(),
            "rerun.testing.components.AffixFuzzer15".into(),
            "rerun.testing.components.AffixFuzzer16".into(),
            "rerun.testing.components.AffixFuzzer17".into(),
            "rerun.testing.components.AffixFuzzer18".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 19usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.testing.components.AffixFuzzer4Indicator".into(),
            "rerun.testing.components.AffixFuzzer1".into(),
            "rerun.testing.components.AffixFuzzer2".into(),
            "rerun.testing.components.AffixFuzzer3".into(),
            "rerun.testing.components.AffixFuzzer4".into(),
            "rerun.testing.components.AffixFuzzer5".into(),
            "rerun.testing.components.AffixFuzzer6".into(),
            "rerun.testing.components.AffixFuzzer7".into(),
            "rerun.testing.components.AffixFuzzer8".into(),
            "rerun.testing.components.AffixFuzzer9".into(),
            "rerun.testing.components.AffixFuzzer10".into(),
            "rerun.testing.components.AffixFuzzer11".into(),
            "rerun.testing.components.AffixFuzzer12".into(),
            "rerun.testing.components.AffixFuzzer13".into(),
            "rerun.testing.components.AffixFuzzer14".into(),
            "rerun.testing.components.AffixFuzzer15".into(),
            "rerun.testing.components.AffixFuzzer16".into(),
            "rerun.testing.components.AffixFuzzer17".into(),
            "rerun.testing.components.AffixFuzzer18".into(),
        ]
    });

impl AffixFuzzer4 {
    /// The total number of components in the archetype: 0 required, 1 recommended, 18 optional
    pub const NUM_COMPONENTS: usize = 19usize;
}

/// Indicator component for the [`AffixFuzzer4`] [`::re_types_core::Archetype`]
pub type AffixFuzzer4Indicator = ::re_types_core::GenericIndicatorComponent<AffixFuzzer4>;

impl ::re_types_core::Archetype for AffixFuzzer4 {
    type Indicator = AffixFuzzer4Indicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.testing.archetypes.AffixFuzzer4".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Affix fuzzer 4"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: AffixFuzzer4Indicator = AffixFuzzer4Indicator::DEFAULT;
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
        let fuzz2101 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer1") {
                Some({
                    <crate::testing::components::AffixFuzzer1>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2101")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2101")?
                })
            } else {
                None
            };
        let fuzz2102 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer2") {
                Some({
                    <crate::testing::components::AffixFuzzer2>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2102")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2102")?
                })
            } else {
                None
            };
        let fuzz2103 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer3") {
                Some({
                    <crate::testing::components::AffixFuzzer3>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2103")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2103")?
                })
            } else {
                None
            };
        let fuzz2104 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer4") {
                Some({
                    <crate::testing::components::AffixFuzzer4>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2104")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2104")?
                })
            } else {
                None
            };
        let fuzz2105 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer5") {
                Some({
                    <crate::testing::components::AffixFuzzer5>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2105")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2105")?
                })
            } else {
                None
            };
        let fuzz2106 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer6") {
                Some({
                    <crate::testing::components::AffixFuzzer6>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2106")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2106")?
                })
            } else {
                None
            };
        let fuzz2107 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer7") {
                Some({
                    <crate::testing::components::AffixFuzzer7>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2107")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2107")?
                })
            } else {
                None
            };
        let fuzz2108 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer8") {
                Some({
                    <crate::testing::components::AffixFuzzer8>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2108")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2108")?
                })
            } else {
                None
            };
        let fuzz2109 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer9") {
                Some({
                    <crate::testing::components::AffixFuzzer9>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2109")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2109")?
                })
            } else {
                None
            };
        let fuzz2110 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer10") {
                Some({
                    <crate::testing::components::AffixFuzzer10>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2110")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2110")?
                })
            } else {
                None
            };
        let fuzz2111 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer11") {
                Some({
                    <crate::testing::components::AffixFuzzer11>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2111")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2111")?
                })
            } else {
                None
            };
        let fuzz2112 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer12") {
                Some({
                    <crate::testing::components::AffixFuzzer12>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2112")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2112")?
                })
            } else {
                None
            };
        let fuzz2113 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer13") {
                Some({
                    <crate::testing::components::AffixFuzzer13>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2113")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2113")?
                })
            } else {
                None
            };
        let fuzz2114 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer14") {
                Some({
                    <crate::testing::components::AffixFuzzer14>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2114")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2114")?
                })
            } else {
                None
            };
        let fuzz2115 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer15") {
                Some({
                    <crate::testing::components::AffixFuzzer15>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2115")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2115")?
                })
            } else {
                None
            };
        let fuzz2116 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer16") {
                Some({
                    <crate::testing::components::AffixFuzzer16>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2116")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2116")?
                })
            } else {
                None
            };
        let fuzz2117 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer17") {
                Some({
                    <crate::testing::components::AffixFuzzer17>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2117")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2117")?
                })
            } else {
                None
            };
        let fuzz2118 =
            if let Some(array) = arrays_by_name.get("rerun.testing.components.AffixFuzzer18") {
                Some({
                    <crate::testing::components::AffixFuzzer18>::from_arrow_opt(&**array)
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2118")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.testing.archetypes.AffixFuzzer4#fuzz2118")?
                })
            } else {
                None
            };
        Ok(Self {
            fuzz2101,
            fuzz2102,
            fuzz2103,
            fuzz2104,
            fuzz2105,
            fuzz2106,
            fuzz2107,
            fuzz2108,
            fuzz2109,
            fuzz2110,
            fuzz2111,
            fuzz2112,
            fuzz2113,
            fuzz2114,
            fuzz2115,
            fuzz2116,
            fuzz2117,
            fuzz2118,
        })
    }
}

impl ::re_types_core::AsComponents for AffixFuzzer4 {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            self.fuzz2101
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2102
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2103
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2104
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2105
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2106
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2107
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2108
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2109
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2110
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2111
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2112
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2113
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2114
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2115
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2116
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2117
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.fuzz2118
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl AffixFuzzer4 {
    /// Create a new `AffixFuzzer4`.
    #[inline]
    pub fn new() -> Self {
        Self {
            fuzz2101: None,
            fuzz2102: None,
            fuzz2103: None,
            fuzz2104: None,
            fuzz2105: None,
            fuzz2106: None,
            fuzz2107: None,
            fuzz2108: None,
            fuzz2109: None,
            fuzz2110: None,
            fuzz2111: None,
            fuzz2112: None,
            fuzz2113: None,
            fuzz2114: None,
            fuzz2115: None,
            fuzz2116: None,
            fuzz2117: None,
            fuzz2118: None,
        }
    }

    #[inline]
    pub fn with_fuzz2101(
        mut self,
        fuzz2101: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer1>>,
    ) -> Self {
        self.fuzz2101 = Some(fuzz2101.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2102(
        mut self,
        fuzz2102: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer2>>,
    ) -> Self {
        self.fuzz2102 = Some(fuzz2102.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2103(
        mut self,
        fuzz2103: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer3>>,
    ) -> Self {
        self.fuzz2103 = Some(fuzz2103.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2104(
        mut self,
        fuzz2104: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer4>>,
    ) -> Self {
        self.fuzz2104 = Some(fuzz2104.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2105(
        mut self,
        fuzz2105: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer5>>,
    ) -> Self {
        self.fuzz2105 = Some(fuzz2105.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2106(
        mut self,
        fuzz2106: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer6>>,
    ) -> Self {
        self.fuzz2106 = Some(fuzz2106.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2107(
        mut self,
        fuzz2107: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer7>>,
    ) -> Self {
        self.fuzz2107 = Some(fuzz2107.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2108(
        mut self,
        fuzz2108: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer8>>,
    ) -> Self {
        self.fuzz2108 = Some(fuzz2108.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2109(
        mut self,
        fuzz2109: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer9>>,
    ) -> Self {
        self.fuzz2109 = Some(fuzz2109.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2110(
        mut self,
        fuzz2110: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer10>>,
    ) -> Self {
        self.fuzz2110 = Some(fuzz2110.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2111(
        mut self,
        fuzz2111: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer11>>,
    ) -> Self {
        self.fuzz2111 = Some(fuzz2111.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2112(
        mut self,
        fuzz2112: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer12>>,
    ) -> Self {
        self.fuzz2112 = Some(fuzz2112.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2113(
        mut self,
        fuzz2113: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer13>>,
    ) -> Self {
        self.fuzz2113 = Some(fuzz2113.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2114(
        mut self,
        fuzz2114: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer14>>,
    ) -> Self {
        self.fuzz2114 = Some(fuzz2114.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2115(
        mut self,
        fuzz2115: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer15>>,
    ) -> Self {
        self.fuzz2115 = Some(fuzz2115.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2116(
        mut self,
        fuzz2116: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer16>>,
    ) -> Self {
        self.fuzz2116 = Some(fuzz2116.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2117(
        mut self,
        fuzz2117: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer17>>,
    ) -> Self {
        self.fuzz2117 = Some(fuzz2117.into_iter().map(Into::into).collect());
        self
    }

    #[inline]
    pub fn with_fuzz2118(
        mut self,
        fuzz2118: impl IntoIterator<Item = impl Into<crate::testing::components::AffixFuzzer18>>,
    ) -> Self {
        self.fuzz2118 = Some(fuzz2118.into_iter().map(Into::into).collect());
        self
    }
}
