// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/visible_time_range.fbs".

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

use crate::external::arrow2;
use crate::SerializationResult;
use crate::{ComponentBatch, ComponentBatchCowWithDescriptor};
use crate::{ComponentDescriptor, ComponentName};
use crate::{DeserializationError, DeserializationResult};

/// **Datatype**: Visible time range bounds for a specific timeline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisibleTimeRange {
    /// Name of the timeline this applies to.
    pub timeline: crate::datatypes::Utf8,

    /// Time range to use for this timeline.
    pub range: crate::datatypes::TimeRange,
}

crate::macros::impl_into_cow!(VisibleTimeRange);

impl crate::Loggable for VisibleTimeRange {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::Struct(Fields::from(vec![
            Field::new(
                "timeline",
                <crate::datatypes::Utf8>::arrow_datatype(),
                false,
            ),
            Field::new(
                "range",
                <crate::datatypes::TimeRange>::arrow_datatype(),
                false,
            ),
        ]))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use crate::{arrow_helpers::as_array_ref, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let fields = Fields::from(vec![
                Field::new(
                    "timeline",
                    <crate::datatypes::Utf8>::arrow_datatype(),
                    false,
                ),
                Field::new(
                    "range",
                    <crate::datatypes::TimeRange>::arrow_datatype(),
                    false,
                ),
            ]);
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            as_array_ref(StructArray::new(
                fields,
                vec![
                    {
                        let (somes, timeline): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.timeline.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let timeline_validity: Option<arrow::buffer::NullBuffer> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            let offsets = arrow::buffer::OffsetBuffer::<i32>::from_lengths(
                                timeline.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.0.len()).unwrap_or_default()
                                }),
                            );
                            let inner_data: arrow::buffer::Buffer = timeline
                                .into_iter()
                                .flatten()
                                .flat_map(|datum| datum.0.into_arrow2_buffer())
                                .collect();
                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                            as_array_ref(unsafe {
                                StringArray::new_unchecked(offsets, inner_data, timeline_validity)
                            })
                        }
                    },
                    {
                        let (somes, range): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.range.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let range_validity: Option<arrow::buffer::NullBuffer> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = range_validity;
                            crate::datatypes::TimeRange::to_arrow_opt(range)?
                        }
                    },
                ],
                validity,
            ))
        })
    }

    fn from_arrow2_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use crate::{Loggable as _, ResultExt as _};
        use arrow::datatypes::*;
        use arrow2::{array::*, buffer::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.VisibleTimeRange")?;
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
                let timeline = {
                    if !arrays_by_name.contains_key("timeline") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "timeline",
                        ))
                        .with_context("rerun.datatypes.VisibleTimeRange");
                    }
                    let arrow_data = &**arrays_by_name["timeline"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<arrow2::array::Utf8Array<i32>>()
                            .ok_or_else(|| {
                                let expected = DataType::Utf8;
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context("rerun.datatypes.VisibleTimeRange#timeline")?;
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
                                res_or_opt
                                    .map(|v| crate::datatypes::Utf8(crate::ArrowString::from(v)))
                            })
                        })
                        .collect::<DeserializationResult<Vec<Option<_>>>>()
                        .with_context("rerun.datatypes.VisibleTimeRange#timeline")?
                        .into_iter()
                    }
                };
                let range = {
                    if !arrays_by_name.contains_key("range") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "range",
                        ))
                        .with_context("rerun.datatypes.VisibleTimeRange");
                    }
                    let arrow_data = &**arrays_by_name["range"];
                    crate::datatypes::TimeRange::from_arrow2_opt(arrow_data)
                        .with_context("rerun.datatypes.VisibleTimeRange#range")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(timeline, range),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(timeline, range)| {
                        Ok(Self {
                            timeline: timeline
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.VisibleTimeRange#timeline")?,
                            range: range
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.VisibleTimeRange#range")?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.VisibleTimeRange")?
            }
        })
    }
}

impl crate::SizeBytes for VisibleTimeRange {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.timeline.heap_size_bytes() + self.range.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Utf8>::is_pod() && <crate::datatypes::TimeRange>::is_pod()
    }
}
