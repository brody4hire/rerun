// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/scalar.fbs".

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
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A double-precision scalar, e.g. for use for time-series plots.
///
/// The current timeline value will be used for the time/X-axis, hence scalars
/// cannot be static.
///
/// When used to produce a plot, this archetype is used to provide the data that
/// is referenced by [`archetypes::SeriesLine`][crate::archetypes::SeriesLine] or [`archetypes::SeriesPoint`][crate::archetypes::SeriesPoint]. You can do
/// this by logging both archetypes to the same path, or alternatively configuring
/// the plot-specific archetypes through the blueprint.
///
/// ## Example
///
/// ### Simple line plot
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_scalar").spawn()?;
///
///     // Log the data on a timeline called "step".
///     for step in 0..64 {
///         rec.set_time_sequence("step", step);
///         rec.log("scalar", &rerun::Scalar::new((step as f64 / 10.0).sin()))?;
///     }
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/1200w.png">
///   <img src="https://static.rerun.io/scalar_simple/8bcc92f56268739f8cd24d60d1fe72a655f62a46/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Scalar {
    /// The scalar value to log.
    pub scalar: crate::components::Scalar,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Scalar".into()),
            component_name: "rerun.components.Scalar".into(),
            archetype_field_name: Some("scalar".into()),
        }]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Scalar".into()),
            component_name: "ScalarIndicator".into(),
            archetype_field_name: None,
        }]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Scalar".into()),
                component_name: "rerun.components.Scalar".into(),
                archetype_field_name: Some("scalar".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.archetypes.Scalar".into()),
                component_name: "ScalarIndicator".into(),
                archetype_field_name: None,
            },
        ]
    });

impl Scalar {
    /// The total number of components in the archetype: 1 required, 1 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 2usize;
}

/// Indicator component for the [`Scalar`] [`::re_types_core::Archetype`]
pub type ScalarIndicator = ::re_types_core::GenericIndicatorComponent<Scalar>;

impl ::re_types_core::Archetype for Scalar {
    type Indicator = ScalarIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Scalar".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Scalar"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: ScalarIndicator = ScalarIndicator::DEFAULT;
        MaybeOwnedComponentBatch::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow2_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let scalar = {
            let array = arrays_by_name
                .get("rerun.components.Scalar")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Scalar#scalar")?;
            <crate::components::Scalar>::from_arrow2_opt(&**array)
                .with_context("rerun.archetypes.Scalar#scalar")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Scalar#scalar")?
        };
        Ok(Self { scalar })
    }
}

impl ::re_types_core::AsComponents for Scalar {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.scalar as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::MaybeOwnedComponentBatch {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.archetypes.Scalar".into()),
                        archetype_field_name: Some(("scalar").into()),
                        component_name: ("rerun.components.Scalar").into(),
                    }),
                }
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Scalar {}

impl Scalar {
    /// Create a new `Scalar`.
    #[inline]
    pub fn new(scalar: impl Into<crate::components::Scalar>) -> Self {
        Self {
            scalar: scalar.into(),
        }
    }
}

impl ::re_types_core::SizeBytes for Scalar {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.scalar.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::components::Scalar>::is_pod()
    }
}
