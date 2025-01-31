// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/tensor_view_fit.fbs".

#include "tensor_view_fit.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {
    TensorViewFit TensorViewFit::clear_fields() {
        auto archetype = TensorViewFit();
        archetype.scaling =
            ComponentBatch::empty<rerun::blueprint::components::ViewFit>(Descriptor_scaling)
                .value_or_throw();
        return archetype;
    }
} // namespace rerun::blueprint::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>>
        AsComponents<blueprint::archetypes::TensorViewFit>::serialize(
            const blueprint::archetypes::TensorViewFit& archetype
        ) {
        using namespace blueprint::archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(2);

        if (archetype.scaling.has_value()) {
            cells.push_back(archetype.scaling.value());
        }
        {
            auto indicator = TensorViewFit::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
