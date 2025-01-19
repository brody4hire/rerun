// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/datatypes/component_column_selector.fbs".

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

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: Describe a component column to be selected in the dataframe view.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ComponentColumnSelector {
    /// The entity path for this component.
    pub entity_path: crate::datatypes::EntityPath,

    /// The name of the component.
    pub component: crate::datatypes::Utf8,
}

::re_types_core::macros::impl_into_cow!(ComponentColumnSelector);

impl ::re_types_core::Loggable for ComponentColumnSelector {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::Struct(Fields::from(vec![
            Field::new(
                "entity_path",
                <crate::datatypes::EntityPath>::arrow_datatype(),
                false,
            ),
            Field::new(
                "component",
                <crate::datatypes::Utf8>::arrow_datatype(),
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
        use ::re_types_core::{arrow_helpers::as_array_ref, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let fields = Fields::from(vec![
                Field::new(
                    "entity_path",
                    <crate::datatypes::EntityPath>::arrow_datatype(),
                    false,
                ),
                Field::new(
                    "component",
                    <crate::datatypes::Utf8>::arrow_datatype(),
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
                        let (somes, entity_path): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.entity_path.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let entity_path_validity: Option<arrow::buffer::NullBuffer> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            let offsets = arrow::buffer::OffsetBuffer::<i32>::from_lengths(
                                entity_path.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.0.len()).unwrap_or_default()
                                }),
                            );
                            #[allow(clippy::unwrap_used)]
                            let capacity = offsets.last().copied().unwrap() as usize;
                            let mut buffer_builder =
                                arrow::array::builder::BufferBuilder::<u8>::new(capacity);
                            for data in entity_path.iter().flatten() {
                                buffer_builder.append_slice(data.0.as_bytes());
                            }
                            let inner_data: arrow::buffer::Buffer = buffer_builder.finish();

                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                            as_array_ref(unsafe {
                                StringArray::new_unchecked(
                                    offsets,
                                    inner_data,
                                    entity_path_validity,
                                )
                            })
                        }
                    },
                    {
                        let (somes, component): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| datum.component.clone());
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let component_validity: Option<arrow::buffer::NullBuffer> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            let offsets = arrow::buffer::OffsetBuffer::<i32>::from_lengths(
                                component.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.0.len()).unwrap_or_default()
                                }),
                            );
                            #[allow(clippy::unwrap_used)]
                            let capacity = offsets.last().copied().unwrap() as usize;
                            let mut buffer_builder =
                                arrow::array::builder::BufferBuilder::<u8>::new(capacity);
                            for data in component.iter().flatten() {
                                buffer_builder.append_slice(data.0.as_bytes());
                            }
                            let inner_data: arrow::buffer::Buffer = buffer_builder.finish();

                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                            as_array_ref(unsafe {
                                StringArray::new_unchecked(offsets, inner_data, component_validity)
                            })
                        }
                    },
                ],
                validity,
            ))
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow::array::StructArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.blueprint.datatypes.ComponentColumnSelector")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.columns());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name().as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let entity_path = {
                    if !arrays_by_name.contains_key("entity_path") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "entity_path",
                        ))
                        .with_context("rerun.blueprint.datatypes.ComponentColumnSelector");
                    }
                    let arrow_data = &**arrays_by_name["entity_path"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<StringArray>()
                            .ok_or_else(|| {
                                let expected = DataType::Utf8;
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context(
                                "rerun.blueprint.datatypes.ComponentColumnSelector#entity_path",
                            )?;
                        let arrow_data_buf = arrow_data.values();
                        let offsets = arrow_data.offsets();
                        ZipValidity::new_with_validity(offsets.windows(2), arrow_data.nulls())
                            .map(|elem| {
                                elem.map(|window| {
                                    let start = window[0] as usize;
                                    let end = window[1] as usize;
                                    let len = end - start;
                                    if arrow_data_buf.len() < end {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_buf.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = arrow_data_buf.slice_with_length(start, len);
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .map(|res_or_opt| {
                                res_or_opt.map(|res_or_opt| {
                                    res_or_opt.map(|v| {
                                        crate::datatypes::EntityPath(
                                            ::re_types_core::ArrowString::from(v),
                                        )
                                    })
                                })
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()
                            .with_context(
                                "rerun.blueprint.datatypes.ComponentColumnSelector#entity_path",
                            )?
                            .into_iter()
                    }
                };
                let component = {
                    if !arrays_by_name.contains_key("component") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "component",
                        ))
                        .with_context("rerun.blueprint.datatypes.ComponentColumnSelector");
                    }
                    let arrow_data = &**arrays_by_name["component"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<StringArray>()
                            .ok_or_else(|| {
                                let expected = DataType::Utf8;
                                let actual = arrow_data.data_type().clone();
                                DeserializationError::datatype_mismatch(expected, actual)
                            })
                            .with_context(
                                "rerun.blueprint.datatypes.ComponentColumnSelector#component",
                            )?;
                        let arrow_data_buf = arrow_data.values();
                        let offsets = arrow_data.offsets();
                        ZipValidity::new_with_validity(offsets.windows(2), arrow_data.nulls())
                            .map(|elem| {
                                elem.map(|window| {
                                    let start = window[0] as usize;
                                    let end = window[1] as usize;
                                    let len = end - start;
                                    if arrow_data_buf.len() < end {
                                        return Err(DeserializationError::offset_slice_oob(
                                            (start, end),
                                            arrow_data_buf.len(),
                                        ));
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    let data = arrow_data_buf.slice_with_length(start, len);
                                    Ok(data)
                                })
                                .transpose()
                            })
                            .map(|res_or_opt| {
                                res_or_opt.map(|res_or_opt| {
                                    res_or_opt.map(|v| {
                                        crate::datatypes::Utf8(::re_types_core::ArrowString::from(
                                            v,
                                        ))
                                    })
                                })
                            })
                            .collect::<DeserializationResult<Vec<Option<_>>>>()
                            .with_context(
                                "rerun.blueprint.datatypes.ComponentColumnSelector#component",
                            )?
                            .into_iter()
                    }
                };
                ZipValidity::new_with_validity(
                    ::itertools::izip!(entity_path, component),
                    arrow_data.nulls(),
                )
                .map(|opt| {
                    opt.map(|(entity_path, component)| {
                        Ok(Self {
                            entity_path: entity_path
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.blueprint.datatypes.ComponentColumnSelector#entity_path",
                                )?,
                            component: component
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context(
                                    "rerun.blueprint.datatypes.ComponentColumnSelector#component",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.blueprint.datatypes.ComponentColumnSelector")?
            }
        })
    }
}

impl ::re_byte_size::SizeBytes for ComponentColumnSelector {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.entity_path.heap_size_bytes() + self.component.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::EntityPath>::is_pod() && <crate::datatypes::Utf8>::is_pod()
    }
}
