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
use crate::{ComponentBatch, MaybeOwnedComponentBatch};
use crate::{ComponentDescriptor, ComponentName};
use crate::{DeserializationError, DeserializationResult};

/// **Datatype**: Left or right boundary of a time range.
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum TimeRangeBoundary {
    /// Boundary is a value relative to the time cursor.
    CursorRelative(crate::datatypes::TimeInt),

    /// Boundary is an absolute value.
    Absolute(crate::datatypes::TimeInt),

    /// The boundary extends to infinity.
    Infinite,
}

crate::macros::impl_into_cow!(TimeRangeBoundary);

impl crate::Loggable for TimeRangeBoundary {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::Union(
            UnionFields::new(
                vec![0, 1, 2, 3],
                vec![
                    Field::new("_null_markers", DataType::Null, true),
                    Field::new(
                        "CursorRelative",
                        <crate::datatypes::TimeInt>::arrow_datatype(),
                        false,
                    ),
                    Field::new(
                        "Absolute",
                        <crate::datatypes::TimeInt>::arrow_datatype(),
                        false,
                    ),
                    Field::new("Infinite", DataType::Null, true),
                ],
            ),
            UnionMode::Dense,
        )
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use crate::{Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};

        #[allow(unused)]
        fn as_array_ref<T: Array + 'static>(t: T) -> ArrayRef {
            std::sync::Arc::new(t) as ArrayRef
        }
        Ok({
            // Dense Arrow union
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            let field_type_ids = [0, 1, 2, 3];
            let fields = vec![
                Field::new("_null_markers", DataType::Null, true),
                Field::new(
                    "CursorRelative",
                    <crate::datatypes::TimeInt>::arrow_datatype(),
                    false,
                ),
                Field::new(
                    "Absolute",
                    <crate::datatypes::TimeInt>::arrow_datatype(),
                    false,
                ),
                Field::new("Infinite", DataType::Null, true),
            ];
            let type_ids: Vec<i8> = data
                .iter()
                .map(|a| match a.as_deref() {
                    None => 0,
                    Some(Self::CursorRelative(_)) => 1i8,
                    Some(Self::Absolute(_)) => 2i8,
                    Some(Self::Infinite) => 3i8,
                })
                .collect();
            let offsets = {
                let mut cursor_relative_offset = 0;
                let mut absolute_offset = 0;
                let mut infinite_offset = 0;
                let mut nulls_offset = 0;
                data.iter()
                    .map(|v| match v.as_deref() {
                        None => {
                            let offset = nulls_offset;
                            nulls_offset += 1;
                            offset
                        }
                        Some(Self::CursorRelative(_)) => {
                            let offset = cursor_relative_offset;
                            cursor_relative_offset += 1;
                            offset
                        }
                        Some(Self::Absolute(_)) => {
                            let offset = absolute_offset;
                            absolute_offset += 1;
                            offset
                        }
                        Some(Self::Infinite) => {
                            let offset = infinite_offset;
                            infinite_offset += 1;
                            offset
                        }
                    })
                    .collect()
            };
            let children = vec![
                as_array_ref(NullArray::new(data.iter().filter(|v| v.is_none()).count())),
                {
                    let cursor_relative: Vec<_> = data
                        .iter()
                        .filter_map(|datum| match datum.as_deref() {
                            Some(Self::CursorRelative(v)) => Some(v.clone()),
                            _ => None,
                        })
                        .collect();
                    let cursor_relative_validity: Option<arrow::buffer::NullBuffer> = None;
                    as_array_ref(PrimitiveArray::<Int64Type>::new(
                        ScalarBuffer::from(
                            cursor_relative
                                .into_iter()
                                .map(|datum| datum.0)
                                .collect::<Vec<_>>(),
                        ),
                        cursor_relative_validity,
                    ))
                },
                {
                    let absolute: Vec<_> = data
                        .iter()
                        .filter_map(|datum| match datum.as_deref() {
                            Some(Self::Absolute(v)) => Some(v.clone()),
                            _ => None,
                        })
                        .collect();
                    let absolute_validity: Option<arrow::buffer::NullBuffer> = None;
                    as_array_ref(PrimitiveArray::<Int64Type>::new(
                        ScalarBuffer::from(
                            absolute
                                .into_iter()
                                .map(|datum| datum.0)
                                .collect::<Vec<_>>(),
                        ),
                        absolute_validity,
                    ))
                },
                as_array_ref(NullArray::new(
                    data.iter()
                        .filter(|datum| matches!(datum.as_deref(), Some(Self::Infinite)))
                        .count(),
                )),
            ];
            debug_assert_eq!(field_type_ids.len(), fields.len());
            debug_assert_eq!(fields.len(), children.len());
            as_array_ref(UnionArray::try_new(
                UnionFields::new(field_type_ids, fields),
                ScalarBuffer::from(type_ids),
                Some(offsets),
                children,
            )?)
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
                .downcast_ref::<arrow2::array::UnionArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.TimeRangeBoundary")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_types, arrow_data_arrays) =
                    (arrow_data.types(), arrow_data.fields());
                let arrow_data_offsets = arrow_data
                    .offsets()
                    .ok_or_else(|| {
                        let expected = Self::arrow_datatype();
                        let actual = arrow_data.data_type().clone();
                        DeserializationError::datatype_mismatch(expected, actual)
                    })
                    .with_context("rerun.datatypes.TimeRangeBoundary")?;
                if arrow_data_types.len() != arrow_data_offsets.len() {
                    return Err(DeserializationError::offset_slice_oob(
                        (0, arrow_data_types.len()),
                        arrow_data_offsets.len(),
                    ))
                    .with_context("rerun.datatypes.TimeRangeBoundary");
                }
                let cursor_relative = {
                    if 1usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[1usize];
                    arrow_data
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .ok_or_else(|| {
                            let expected = DataType::Int64;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.TimeRangeBoundary#CursorRelative")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .map(|res_or_opt| res_or_opt.map(crate::datatypes::TimeInt))
                        .collect::<Vec<_>>()
                };
                let absolute = {
                    if 2usize >= arrow_data_arrays.len() {
                        return Ok(Vec::new());
                    }
                    let arrow_data = &*arrow_data_arrays[2usize];
                    arrow_data
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .ok_or_else(|| {
                            let expected = DataType::Int64;
                            let actual = arrow_data.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.TimeRangeBoundary#Absolute")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .map(|res_or_opt| res_or_opt.map(crate::datatypes::TimeInt))
                        .collect::<Vec<_>>()
                };
                arrow_data_types
                    .iter()
                    .enumerate()
                    .map(|(i, typ)| {
                        let offset = arrow_data_offsets[i];
                        if *typ == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(match typ {
                                1i8 => Self::CursorRelative({
                                    if offset as usize >= cursor_relative.len() {
                                        return Err(DeserializationError::offset_oob(
                                            offset as _,
                                            cursor_relative.len(),
                                        ))
                                        .with_context(
                                            "rerun.datatypes.TimeRangeBoundary#CursorRelative",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe { cursor_relative.get_unchecked(offset as usize) }
                                        .clone()
                                        .ok_or_else(DeserializationError::missing_data)
                                        .with_context(
                                            "rerun.datatypes.TimeRangeBoundary#CursorRelative",
                                        )?
                                }),
                                2i8 => Self::Absolute({
                                    if offset as usize >= absolute.len() {
                                        return Err(DeserializationError::offset_oob(
                                            offset as _,
                                            absolute.len(),
                                        ))
                                        .with_context(
                                            "rerun.datatypes.TimeRangeBoundary#Absolute",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe { absolute.get_unchecked(offset as usize) }
                                        .clone()
                                        .ok_or_else(DeserializationError::missing_data)
                                        .with_context(
                                            "rerun.datatypes.TimeRangeBoundary#Absolute",
                                        )?
                                }),
                                3i8 => Self::Infinite,
                                _ => {
                                    return Err(DeserializationError::missing_union_arm(
                                        Self::arrow_datatype(),
                                        "<invalid>",
                                        *typ as _,
                                    ));
                                }
                            }))
                        }
                    })
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.datatypes.TimeRangeBoundary")?
            }
        })
    }
}

impl crate::SizeBytes for TimeRangeBoundary {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        #![allow(clippy::match_same_arms)]
        match self {
            Self::CursorRelative(v) => v.heap_size_bytes(),
            Self::Absolute(v) => v.heap_size_bytes(),
            Self::Infinite => 0,
        }
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::TimeInt>::is_pod() && <crate::datatypes::TimeInt>::is_pod()
    }
}
