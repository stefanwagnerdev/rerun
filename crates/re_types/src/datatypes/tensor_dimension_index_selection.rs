// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/tensor_dimension_selection.fbs".

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

/// **Datatype**: Indexing a specific tensor dimension.
///
/// Selecting `dimension=2` and `index=42` is similar to doing `tensor[:, :, 42, :, :, …]` in numpy.
#[derive(Clone, Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct TensorDimensionIndexSelection {
    /// The dimension number to select.
    pub dimension: u32,

    /// The index along the dimension to use.
    pub index: u64,
}

impl ::re_types_core::SizeBytes for TensorDimensionIndexSelection {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.dimension.heap_size_bytes() + self.index.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <u32>::is_pod() && <u64>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(TensorDimensionIndexSelection);

impl ::re_types_core::Loggable for TensorDimensionIndexSelection {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.TensorDimensionIndexSelection".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new("dimension", DataType::UInt32, false),
            Field::new("index", DataType::UInt64, false),
        ]))
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
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                Self::arrow_datatype(),
                vec![
                    {
                        let (somes, dimension): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.dimension.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let dimension_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt32,
                            dimension
                                .into_iter()
                                .map(|v| v.unwrap_or_default())
                                .collect(),
                            dimension_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, index): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.index.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let index_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt64,
                            index.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            index_bitmap,
                        )
                        .boxed()
                    },
                ],
                bitmap,
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
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.TensorDimensionIndexSelection")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let dimension = {
                    if !arrays_by_name.contains_key("dimension") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "dimension",
                        ))
                        .with_context("rerun.datatypes.TensorDimensionIndexSelection");
                    }
                    let arrow_data = &**arrays_by_name["dimension"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt32Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt32;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.TensorDimensionIndexSelection#dimension")?
                        .into_iter()
                        .map(|opt| opt.copied())
                };
                let index = {
                    if !arrays_by_name.contains_key("index") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "index",
                        ))
                        .with_context("rerun.datatypes.TensorDimensionIndexSelection");
                    }
                    let arrow_data = &**arrays_by_name["index"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt64Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt64;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.TensorDimensionIndexSelection#index")?
                        .into_iter()
                        .map(|opt| opt.copied())
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(dimension, index),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(dimension, index)| {
                        Ok(Self {
                            dimension: dimension
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.datatypes.TensorDimensionIndexSelection#dimension",
                                )?,
                            index: index
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.datatypes.TensorDimensionIndexSelection#index",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.TensorDimensionIndexSelection")?
            }
        })
    }
}
