// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/scalar_axis.fbs".

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

/// **Archetype**: Configuration for the scalar axis of a plot.
#[derive(Clone, Debug, Default)]
pub struct ScalarAxis {
    /// The range of the axis.
    ///
    /// If unset, the range well be automatically determined based on the queried data.
    pub range: Option<crate::components::Range1D>,

    /// If enabled, the Y axis range will remain locked to the specified range when zooming.
    pub zoom_lock: Option<crate::blueprint::components::LockRangeDuringZoom>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ScalarAxis".into()),
            component_name: "ScalarAxisIndicator".into(),
            archetype_field_name: None,
        }]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ScalarAxis".into()),
                component_name: "rerun.components.Range1D".into(),
                archetype_field_name: Some("range".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ScalarAxis".into()),
                component_name: "rerun.blueprint.components.LockRangeDuringZoom".into(),
                archetype_field_name: Some("zoom_lock".into()),
            },
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ScalarAxis".into()),
                component_name: "ScalarAxisIndicator".into(),
                archetype_field_name: None,
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ScalarAxis".into()),
                component_name: "rerun.components.Range1D".into(),
                archetype_field_name: Some("range".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ScalarAxis".into()),
                component_name: "rerun.blueprint.components.LockRangeDuringZoom".into(),
                archetype_field_name: Some("zoom_lock".into()),
            },
        ]
    });

impl ScalarAxis {
    /// The total number of components in the archetype: 0 required, 1 recommended, 2 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`ScalarAxis`] [`::re_types_core::Archetype`]
pub type ScalarAxisIndicator = ::re_types_core::GenericIndicatorComponent<ScalarAxis>;

impl ::re_types_core::Archetype for ScalarAxis {
    type Indicator = ScalarAxisIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ScalarAxis".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Scalar axis"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: ScalarAxisIndicator = ScalarAxisIndicator::DEFAULT;
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
        let range = if let Some(array) = arrays_by_name.get("rerun.components.Range1D") {
            <crate::components::Range1D>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ScalarAxis#range")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let zoom_lock = if let Some(array) =
            arrays_by_name.get("rerun.blueprint.components.LockRangeDuringZoom")
        {
            <crate::blueprint::components::LockRangeDuringZoom>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ScalarAxis#zoom_lock")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { range, zoom_lock })
    }
}

impl ::re_types_core::AsComponents for ScalarAxis {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (self
                .range
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedComponentBatch {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ScalarAxis".into()),
                    archetype_field_name: Some(("range").into()),
                    component_name: ("rerun.components.Range1D").into(),
                }),
            }),
            (self
                .zoom_lock
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedComponentBatch {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ScalarAxis".into()),
                    archetype_field_name: Some(("zoom_lock").into()),
                    component_name: ("rerun.blueprint.components.LockRangeDuringZoom").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ScalarAxis {}

impl ScalarAxis {
    /// Create a new `ScalarAxis`.
    #[inline]
    pub fn new() -> Self {
        Self {
            range: None,
            zoom_lock: None,
        }
    }

    /// The range of the axis.
    ///
    /// If unset, the range well be automatically determined based on the queried data.
    #[inline]
    pub fn with_range(mut self, range: impl Into<crate::components::Range1D>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// If enabled, the Y axis range will remain locked to the specified range when zooming.
    #[inline]
    pub fn with_zoom_lock(
        mut self,
        zoom_lock: impl Into<crate::blueprint::components::LockRangeDuringZoom>,
    ) -> Self {
        self.zoom_lock = Some(zoom_lock.into());
        self
    }
}

impl ::re_types_core::SizeBytes for ScalarAxis {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.range.heap_size_bytes() + self.zoom_lock.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Option<crate::components::Range1D>>::is_pod()
            && <Option<crate::blueprint::components::LockRangeDuringZoom>>::is_pod()
    }
}
