// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/plot_legend.fbs".

#include "plot_legend.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {
    PlotLegend PlotLegend::clear_fields() {
        auto archetype = PlotLegend();
        archetype.corner =
            ComponentBatch::empty<rerun::blueprint::components::Corner2D>(Descriptor_corner)
                .value_or_throw();
        archetype.visible =
            ComponentBatch::empty<rerun::blueprint::components::Visible>(Descriptor_visible)
                .value_or_throw();
        return archetype;
    }
} // namespace rerun::blueprint::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<blueprint::archetypes::PlotLegend>::serialize(
        const blueprint::archetypes::PlotLegend& archetype
    ) {
        using namespace blueprint::archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(3);

        if (archetype.corner.has_value()) {
            cells.push_back(archetype.corner.value());
        }
        if (archetype.visible.has_value()) {
            cells.push_back(archetype.visible.value());
        }
        {
            auto indicator = PlotLegend::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
