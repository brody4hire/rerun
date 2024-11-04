// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/geo_points.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/color.hpp"
#include "../components/lat_lon.hpp"
#include "../components/radius.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: Geospatial points with positions expressed in [EPSG:4326](https://epsg.io/4326) altitude and longitude (North/East-positive degrees), and optional colors and radii.
    ///
    /// **Note**: Geospatial entities are experimental.
    ///
    /// ## Example
    ///
    /// ### Log a geospatial point
    /// ![image](https://static.rerun.io/geopoint_simple/146b0783c5aea1c1b6a9aab938879942b7c820e2/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_geo_points");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     rec.log(
    ///         "rerun_hq",
    ///         rerun::GeoPoints({{59.319221, 18.075631}})
    ///             .with_radii(rerun::Radius::ui_points(10.0f))
    ///             .with_colors(rerun::Color(255, 0, 0))
    ///     );
    /// }
    /// ```
    struct GeoPoints {
        /// The [EPSG:4326](https://epsg.io/4326) coordinates for the points (North/East-positive degrees).
        Collection<rerun::components::LatLon> positions;

        /// Optional radii for the points, effectively turning them into circles.
        std::optional<Collection<rerun::components::Radius>> radii;

        /// Optional colors for the points.
        std::optional<Collection<rerun::components::Color>> colors;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.GeoPointsIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public:
        GeoPoints() = default;
        GeoPoints(GeoPoints&& other) = default;

        explicit GeoPoints(Collection<rerun::components::LatLon> _positions)
            : positions(std::move(_positions)) {}

        /// Optional radii for the points, effectively turning them into circles.
        GeoPoints with_radii(Collection<rerun::components::Radius> _radii) && {
            radii = std::move(_radii);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the points.
        GeoPoints with_colors(Collection<rerun::components::Color> _colors) && {
            colors = std::move(_colors);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::GeoPoints> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(const archetypes::GeoPoints& archetype
        );
    };
} // namespace rerun
