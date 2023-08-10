// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// Representation of a 3D affine transform.
///
/// Rarely used directly, prefer using the underlying representation classes and pass them
/// directly to `Transform3D::child_from_parent` or `Transform3D::parent_from_child`.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Transform3D {
    TranslationAndMat3X3(crate::datatypes::TranslationAndMat3x3),
    TranslationRotationScale(crate::datatypes::TranslationRotationScale3D),
}

impl<'a> From<Transform3D> for ::std::borrow::Cow<'a, Transform3D> {
    #[inline]
    fn from(value: Transform3D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Transform3D> for ::std::borrow::Cow<'a, Transform3D> {
    #[inline]
    fn from(value: &'a Transform3D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for Transform3D {
    type Name = crate::DatatypeName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = <Vec<Self::Item<'a>> as IntoIterator>::IntoIter;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.Transform3D".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Union(
            vec![
                Field {
                    name: "_null_markers".to_owned(),
                    data_type: DataType::Null,
                    is_nullable: true,
                    metadata: [].into(),
                },
                Field {
                    name: "TranslationAndMat3x3".to_owned(),
                    data_type: <crate::datatypes::TranslationAndMat3x3>::to_arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "TranslationRotationScale".to_owned(),
                    data_type: <crate::datatypes::TranslationRotationScale3D>::to_arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                },
            ],
            Some(vec![0i32, 1i32, 2i32]),
            UnionMode::Dense,
        )
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            UnionArray::new(
                (if let Some(ext) = extension_wrapper {
                    DataType::Extension(
                        ext.to_owned(),
                        Box::new(<crate::datatypes::Transform3D>::to_arrow_datatype()),
                        None,
                    )
                } else {
                    <crate::datatypes::Transform3D>::to_arrow_datatype()
                })
                .to_logical_type()
                .clone(),
                data.iter()
                    .map(|a| match a.as_deref() {
                        None => 0,
                        Some(Transform3D::TranslationAndMat3X3(_)) => 1i8,
                        Some(Transform3D::TranslationRotationScale(_)) => 2i8,
                    })
                    .collect(),
                vec![
                    NullArray::new(DataType::Null, data.iter().filter(|v| v.is_none()).count())
                        .boxed(),
                    {
                        let (somes, translation_and_mat_3_x_3): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| {
                                matches!(
                                    datum.as_deref(),
                                    Some(Transform3D::TranslationAndMat3X3(_))
                                )
                            })
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(Transform3D::TranslationAndMat3X3(v)) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let translation_and_mat_3_x_3_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = translation_and_mat_3_x_3_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::TranslationAndMat3x3::try_to_arrow_opt(
                                translation_and_mat_3_x_3,
                                None::<&str>,
                            )?
                        }
                    },
                    {
                        let (somes, translation_rotation_scale): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| {
                                matches!(
                                    datum.as_deref(),
                                    Some(Transform3D::TranslationRotationScale(_))
                                )
                            })
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(Transform3D::TranslationRotationScale(v)) => {
                                        Some(v.clone())
                                    }
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let translation_rotation_scale_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = translation_rotation_scale_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::TranslationRotationScale3D::try_to_arrow_opt(
                                translation_rotation_scale,
                                None::<&str>,
                            )?
                        }
                    },
                ],
                Some({
                    let mut translation_and_mat_3_x_3_offset = 0;
                    let mut translation_rotation_scale_offset = 0;
                    let mut nulls_offset = 0;
                    data.iter()
                        .map(|v| match v.as_deref() {
                            None => {
                                let offset = nulls_offset;
                                nulls_offset += 1;
                                offset
                            }
                            Some(Transform3D::TranslationAndMat3X3(_)) => {
                                let offset = translation_and_mat_3_x_3_offset;
                                translation_and_mat_3_x_3_offset += 1;
                                offset
                            }
                            Some(Transform3D::TranslationRotationScale(_)) => {
                                let offset = translation_rotation_scale_offset;
                                translation_rotation_scale_offset += 1;
                                offset
                            }
                        })
                        .collect()
                }),
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::UnionArray>()
                .ok_or_else(|| {
                    crate::DeserializationError::datatype_mismatch(
                        DataType::Union(
                            vec![
                            Field { name : "_null_markers".to_owned(), data_type :
                            DataType::Null, is_nullable : true, metadata : [].into(), },
                            Field { name : "TranslationAndMat3x3".to_owned(), data_type :
                            < crate ::datatypes::TranslationAndMat3x3 >
                            ::to_arrow_datatype(), is_nullable : false, metadata : []
                            .into(), }, Field { name : "TranslationRotationScale"
                            .to_owned(), data_type : < crate
                            ::datatypes::TranslationRotationScale3D >
                            ::to_arrow_datatype(), is_nullable : false, metadata : []
                            .into(), },
                        ],
                            Some(vec![0i32, 1i32, 2i32]),
                            UnionMode::Dense,
                        ),
                        data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.Transform3D")?;
            if data.is_empty() {
                Vec::new()
            } else {
                let (data_types, data_arrays) = (data.types(), data.fields());
                let data_offsets = data
                    .offsets()
                    .ok_or_else(|| {
                        crate::DeserializationError::datatype_mismatch(
                            DataType::Union(
                                vec![
                                Field { name : "_null_markers".to_owned(), data_type :
                                DataType::Null, is_nullable : true, metadata : [].into(), },
                                Field { name : "TranslationAndMat3x3".to_owned(), data_type
                                : < crate ::datatypes::TranslationAndMat3x3 >
                                ::to_arrow_datatype(), is_nullable : false, metadata : []
                                .into(), }, Field { name : "TranslationRotationScale"
                                .to_owned(), data_type : < crate
                                ::datatypes::TranslationRotationScale3D >
                                ::to_arrow_datatype(), is_nullable : false, metadata : []
                                .into(), },
                            ],
                                Some(vec![0i32, 1i32, 2i32]),
                                UnionMode::Dense,
                            ),
                            data.data_type().clone(),
                        )
                    })
                    .with_context("rerun.datatypes.Transform3D")?;
                if data_types.len() > data_offsets.len() {
                    return Err(crate::DeserializationError::offsets_mismatch(
                        (0, data_types.len()),
                        data_offsets.len(),
                    ))
                    .with_context("rerun.datatypes.Transform3D");
                }
                let translation_and_mat_3_x_3 = {
                    if 1usize >= data_arrays.len() {
                        return Err(
                                crate::DeserializationError::missing_union_arm(
                                    DataType::Union(
                                        vec![
                                            Field { name : "_null_markers".to_owned(), data_type :
                                            DataType::Null, is_nullable : true, metadata : [].into(), },
                                            Field { name : "TranslationAndMat3x3".to_owned(), data_type
                                            : < crate ::datatypes::TranslationAndMat3x3 >
                                            ::to_arrow_datatype(), is_nullable : false, metadata : []
                                            .into(), }, Field { name : "TranslationRotationScale"
                                            .to_owned(), data_type : < crate
                                            ::datatypes::TranslationRotationScale3D >
                                            ::to_arrow_datatype(), is_nullable : false, metadata : []
                                            .into(), },
                                        ],
                                        Some(vec![0i32, 1i32, 2i32,]),
                                        UnionMode::Dense,
                                    ),
                                    "rerun.datatypes.Transform3D#TranslationAndMat3x3",
                                    1usize,
                                ),
                            )
                            .with_context("rerun.datatypes.Transform3D");
                    }
                    let data = &*data_arrays[1usize];
                    crate::datatypes::TranslationAndMat3x3::try_from_arrow_opt(data)
                        .with_context("rerun.datatypes.Transform3D#TranslationAndMat3x3")?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                let translation_rotation_scale = {
                    if 2usize >= data_arrays.len() {
                        return Err(
                                crate::DeserializationError::missing_union_arm(
                                    DataType::Union(
                                        vec![
                                            Field { name : "_null_markers".to_owned(), data_type :
                                            DataType::Null, is_nullable : true, metadata : [].into(), },
                                            Field { name : "TranslationAndMat3x3".to_owned(), data_type
                                            : < crate ::datatypes::TranslationAndMat3x3 >
                                            ::to_arrow_datatype(), is_nullable : false, metadata : []
                                            .into(), }, Field { name : "TranslationRotationScale"
                                            .to_owned(), data_type : < crate
                                            ::datatypes::TranslationRotationScale3D >
                                            ::to_arrow_datatype(), is_nullable : false, metadata : []
                                            .into(), },
                                        ],
                                        Some(vec![0i32, 1i32, 2i32,]),
                                        UnionMode::Dense,
                                    ),
                                    "rerun.datatypes.Transform3D#TranslationRotationScale",
                                    2usize,
                                ),
                            )
                            .with_context("rerun.datatypes.Transform3D");
                    }
                    let data = &*data_arrays[2usize];
                    crate::datatypes::TranslationRotationScale3D::try_from_arrow_opt(data)
                        .with_context("rerun.datatypes.Transform3D#TranslationRotationScale")?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                data_types
                    .iter()
                    .enumerate()
                    .map(|(i, typ)| {
                        let offset = data_offsets[i];
                        if *typ == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(match typ {
                                1i8 => Transform3D::TranslationAndMat3X3({
                                    if offset as usize >= translation_and_mat_3_x_3.len() {
                                        return Err(crate::DeserializationError::offsets_mismatch(
                                            (offset as _, offset as _),
                                            translation_and_mat_3_x_3.len(),
                                        ))
                                        .with_context(
                                            "rerun.datatypes.Transform3D#TranslationAndMat3x3",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe {
                                        translation_and_mat_3_x_3.get_unchecked(offset as usize)
                                    }
                                    .clone()
                                    .unwrap()
                                }),
                                2i8 => Transform3D::TranslationRotationScale({
                                    if offset as usize >= translation_rotation_scale.len() {
                                        return Err(crate::DeserializationError::offsets_mismatch(
                                            (offset as _, offset as _),
                                            translation_rotation_scale.len(),
                                        ))
                                        .with_context(
                                            "rerun.datatypes.Transform3D#TranslationRotationScale",
                                        );
                                    }

                                    #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                    unsafe {
                                        translation_rotation_scale.get_unchecked(offset as usize)
                                    }
                                    .clone()
                                    .unwrap()
                                }),
                                _ => unreachable!(),
                            }))
                        }
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .with_context("rerun.datatypes.Transform3D")?
            }
        })
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        Ok(Self::try_from_arrow_opt(data)?.into_iter())
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Datatype for Transform3D {}
