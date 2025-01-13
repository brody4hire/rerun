// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/geo_line_strings.fbs".

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

/// **Archetype**: Geospatial line strings with positions expressed in [EPSG:4326](https://epsg.io/4326) altitude and longitude (North/East-positive degrees), and optional colors and radii.
///
/// Also known as "line strips" or "polylines".
///
/// ## Example
///
/// ### Log a geospatial line string
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_geo_line_strings").spawn()?;
///
///     rec.log(
///         "colorado",
///         &rerun::GeoLineStrings::from_lat_lon([[
///             [41.0000, -109.0452],
///             [41.0000, -102.0415],
///             [36.9931, -102.0415],
///             [36.9931, -109.0452],
///             [41.0000, -109.0452],
///         ]])
///         .with_radii([rerun::Radius::new_ui_points(2.0)])
///         .with_colors([rerun::Color::from_rgb(0, 0, 255)]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/1200w.png">
///   <img src="https://static.rerun.io/geo_line_strings_simple/5669983eb10906ace303755b5b5039cad75b917f/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct GeoLineStrings {
    /// The line strings, expressed in [EPSG:4326](https://epsg.io/4326) coordinates (North/East-positive degrees).
    pub line_strings: Vec<crate::components::GeoLineString>,

    /// Optional radii for the line strings.
    ///
    /// *Note*: scene units radiii are interpreted as meters. Currently, the display scale only considers the latitude of
    /// the first vertex of each line string (see [this issue](https://github.com/rerun-io/rerun/issues/8013)).
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the line strings.
    pub colors: Option<Vec<crate::components::Color>>,
}

impl GeoLineStrings {
    /// Returns the [`ComponentDescriptor`] for [`Self::line_strings`].
    #[inline]
    pub fn descriptor_line_strings() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GeoLineStrings".into()),
            component_name: "rerun.components.GeoLineString".into(),
            archetype_field_name: Some("line_strings".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::radii`].
    #[inline]
    pub fn descriptor_radii() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GeoLineStrings".into()),
            component_name: "rerun.components.Radius".into(),
            archetype_field_name: Some("radii".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::colors`].
    #[inline]
    pub fn descriptor_colors() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GeoLineStrings".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("colors".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.GeoLineStrings".into()),
            component_name: "rerun.components.GeoLineStringsIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [GeoLineStrings::descriptor_line_strings()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            GeoLineStrings::descriptor_radii(),
            GeoLineStrings::descriptor_colors(),
            GeoLineStrings::descriptor_indicator(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 0usize]> =
    once_cell::sync::Lazy::new(|| []);

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 4usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            GeoLineStrings::descriptor_line_strings(),
            GeoLineStrings::descriptor_radii(),
            GeoLineStrings::descriptor_colors(),
            GeoLineStrings::descriptor_indicator(),
        ]
    });

impl GeoLineStrings {
    /// The total number of components in the archetype: 1 required, 3 recommended, 0 optional
    pub const NUM_COMPONENTS: usize = 4usize;
}

/// Indicator component for the [`GeoLineStrings`] [`::re_types_core::Archetype`]
pub type GeoLineStringsIndicator = ::re_types_core::GenericIndicatorComponent<GeoLineStrings>;

impl ::re_types_core::Archetype for GeoLineStrings {
    type Indicator = GeoLineStringsIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.GeoLineStrings".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Geo line strings"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: GeoLineStringsIndicator = GeoLineStringsIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
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
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let line_strings = {
            let array = arrays_by_descr
                .get(&Self::descriptor_line_strings())
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.GeoLineStrings#line_strings")?;
            <crate::components::GeoLineString>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.GeoLineStrings#line_strings")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.GeoLineStrings#line_strings")?
        };
        let radii = if let Some(array) = arrays_by_descr.get(&Self::descriptor_radii()) {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.GeoLineStrings#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GeoLineStrings#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_descr.get(&Self::descriptor_colors()) {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.GeoLineStrings#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.GeoLineStrings#colors")?
            })
        } else {
            None
        };
        Ok(Self {
            line_strings,
            radii,
            colors,
        })
    }
}

impl ::re_types_core::AsComponents for GeoLineStrings {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.line_strings as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(Self::descriptor_line_strings()),
                }
            }),
            (self
                .radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_radii()),
            }),
            (self
                .colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(Self::descriptor_colors()),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for GeoLineStrings {}

impl GeoLineStrings {
    /// Create a new `GeoLineStrings`.
    #[inline]
    pub(crate) fn new(
        line_strings: impl IntoIterator<Item = impl Into<crate::components::GeoLineString>>,
    ) -> Self {
        Self {
            line_strings: line_strings.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
        }
    }

    /// Optional radii for the line strings.
    ///
    /// *Note*: scene units radiii are interpreted as meters. Currently, the display scale only considers the latitude of
    /// the first vertex of each line string (see [this issue](https://github.com/rerun-io/rerun/issues/8013)).
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the line strings.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }
}

impl ::re_byte_size::SizeBytes for GeoLineStrings {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.line_strings.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.colors.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::GeoLineString>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
    }
}
