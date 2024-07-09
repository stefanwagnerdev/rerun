// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs".

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AffixFuzzer20 {
    pub p: crate::testing::datatypes::PrimitiveComponent,
    pub s: crate::testing::datatypes::StringComponent,
}

impl ::re_types_core::SizeBytes for AffixFuzzer20 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.p.heap_size_bytes() + self.s.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::testing::datatypes::PrimitiveComponent>::is_pod()
            && <crate::testing::datatypes::StringComponent>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(AffixFuzzer20);

impl ::re_types_core::Loggable for AffixFuzzer20 {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.testing.datatypes.AffixFuzzer20".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field::new(
                "p",
                <crate::testing::datatypes::PrimitiveComponent>::arrow_datatype(),
                false,
            ),
            Field::new(
                "s",
                <crate::testing::datatypes::StringComponent>::arrow_datatype(),
                false,
            ),
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
                        let (somes, p): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.p.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let p_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt32,
                            p.into_iter()
                                .map(|datum| datum.map(|datum| datum.0).unwrap_or_default())
                                .collect(),
                            p_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, s): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.s.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let s_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            let offsets = arrow2::offset::Offsets::<i32>::try_from_lengths(
                                s.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.0.len()).unwrap_or_default()
                                }),
                            )?
                            .into();
                            let inner_data: arrow2::buffer::Buffer<u8> = s
                                .into_iter()
                                .flatten()
                                .flat_map(|datum| datum.0 .0)
                                .collect();

                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                            unsafe {
                                Utf8Array::<i32>::new_unchecked(
                                    DataType::Utf8,
                                    offsets,
                                    inner_data,
                                    s_bitmap,
                                )
                            }
                            .boxed()
                        }
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
                .with_context("rerun.testing.datatypes.AffixFuzzer20")?;
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
                let p = {
                    if !arrays_by_name.contains_key("p") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "p",
                        ))
                        .with_context("rerun.testing.datatypes.AffixFuzzer20");
                    }
                    let arrow_data = &**arrays_by_name["p"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt32Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt32;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.testing.datatypes.AffixFuzzer20#p")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .map(|res_or_opt| {
                            res_or_opt.map(crate::testing::datatypes::PrimitiveComponent)
                        })
                };
                let s = {
                    if !arrays_by_name.contains_key("s") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "s",
                        ))
                        .with_context("rerun.testing.datatypes.AffixFuzzer20");
                    }
                    let arrow_data = &**arrays_by_name["s"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::Utf8Array<i32>>()
                            .ok_or_else(|| {
                                let expected = DataType::Utf8;
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context("rerun.testing.datatypes.AffixFuzzer20#s")?;
                        let arrow_data_buf = arrow_data.values();
                        let offsets = arrow_data.offsets();
                        arrow2::bitmap::utils::ZipValidity::new_with_validity(
                            offsets.iter().zip(offsets.lengths()),
                            arrow_data.validity(),
                        )
                        .map(|elem| {
                            elem.map(|(start, len)| {
                                let start = *start as usize;
                                let end = start + len;
                                if end > arrow_data_buf.len() {
                                    return Err(DeserializationError::offset_slice_oob(
                                        (start, end),
                                        arrow_data_buf.len(),
                                    ));
                                }

                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                let data =
                                    unsafe { arrow_data_buf.clone().sliced_unchecked(start, len) };
                                Ok(data)
                            })
                            .transpose()
                        })
                        .map(|res_or_opt| {
                            res_or_opt.map(|res_or_opt| {
                                res_or_opt.map(|v| {
                                    crate::testing::datatypes::StringComponent(
                                        ::re_types_core::ArrowString(v),
                                    )
                                })
                            })
                        })
                        .collect::<DeserializationResult<Vec<Option<_>>>>()
                        .with_context("rerun.testing.datatypes.AffixFuzzer20#s")?
                        .into_iter()
                    }
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(p, s),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(p, s)| {
                        Ok(Self {
                            p: p.ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.testing.datatypes.AffixFuzzer20#p")?,
                            s: s.ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.testing.datatypes.AffixFuzzer20#s")?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.testing.datatypes.AffixFuzzer20")?
            }
        })
    }
}
