// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/background.fbs".

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

/// **Archetype**: Configuration for the background of a view.
#[derive(Clone, Debug, Copy)]
pub struct Background {
    /// The type of the background.
    pub kind: crate::blueprint::components::BackgroundKind,

    /// Color used for the `SolidColor` background type.
    pub color: Option<crate::components::Color>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.Background".into()),
            component_name: "rerun.blueprint.components.BackgroundKind".into(),
            archetype_field_name: Some("kind".into()),
        }]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.Background".into()),
            component_name: "BackgroundIndicator".into(),
            archetype_field_name: None,
        }]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.Background".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("color".into()),
        }]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.Background".into()),
                component_name: "rerun.blueprint.components.BackgroundKind".into(),
                archetype_field_name: Some("kind".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.Background".into()),
                component_name: "BackgroundIndicator".into(),
                archetype_field_name: None,
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.Background".into()),
                component_name: "rerun.components.Color".into(),
                archetype_field_name: Some("color".into()),
            },
        ]
    });

impl Background {
    /// The total number of components in the archetype: 1 required, 1 recommended, 1 optional
    pub const NUM_COMPONENTS: usize = 3usize;
}

/// Indicator component for the [`Background`] [`::re_types_core::Archetype`]
pub type BackgroundIndicator = ::re_types_core::GenericIndicatorComponent<Background>;

impl ::re_types_core::Archetype for Background {
    type Indicator = BackgroundIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.Background".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Background"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: BackgroundIndicator = BackgroundIndicator::DEFAULT;
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
        let kind = {
            let array = arrays_by_name
                .get("rerun.blueprint.components.BackgroundKind")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.Background#kind")?;
            <crate::blueprint::components::BackgroundKind>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.Background#kind")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.Background#kind")?
        };
        let color = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            <crate::components::Color>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.Background#color")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        Ok(Self { kind, color })
    }
}

impl ::re_types_core::AsComponents for Background {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.kind as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::MaybeOwnedComponentBatch {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some("rerun.blueprint.archetypes.Background".into()),
                        archetype_field_name: Some(("kind").into()),
                        component_name: ("rerun.blueprint.components.BackgroundKind").into(),
                    }),
                }
            }),
            (self
                .color
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::MaybeOwnedComponentBatch {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.Background".into()),
                    archetype_field_name: Some(("color").into()),
                    component_name: ("rerun.components.Color").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Background {}

impl Background {
    /// Create a new `Background`.
    #[inline]
    pub fn new(kind: impl Into<crate::blueprint::components::BackgroundKind>) -> Self {
        Self {
            kind: kind.into(),
            color: None,
        }
    }

    /// Color used for the `SolidColor` background type.
    #[inline]
    pub fn with_color(mut self, color: impl Into<crate::components::Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl ::re_types_core::SizeBytes for Background {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.kind.heap_size_bytes() + self.color.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::blueprint::components::BackgroundKind>::is_pod()
            && <Option<crate::components::Color>>::is_pod()
    }
}
