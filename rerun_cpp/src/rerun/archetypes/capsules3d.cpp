// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/capsules3d.fbs".

#include "capsules3d.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    Capsules3D Capsules3D::clear_fields() {
        auto archetype = Capsules3D();
        archetype.lengths =
            ComponentBatch::empty<rerun::components::Length>(Descriptor_lengths).value_or_throw();
        archetype.radii =
            ComponentBatch::empty<rerun::components::Radius>(Descriptor_radii).value_or_throw();
        archetype.translations =
            ComponentBatch::empty<rerun::components::PoseTranslation3D>(Descriptor_translations)
                .value_or_throw();
        archetype.rotation_axis_angles =
            ComponentBatch::empty<rerun::components::PoseRotationAxisAngle>(
                Descriptor_rotation_axis_angles
            )
                .value_or_throw();
        archetype.quaternions =
            ComponentBatch::empty<rerun::components::PoseRotationQuat>(Descriptor_quaternions)
                .value_or_throw();
        archetype.colors =
            ComponentBatch::empty<rerun::components::Color>(Descriptor_colors).value_or_throw();
        archetype.labels =
            ComponentBatch::empty<rerun::components::Text>(Descriptor_labels).value_or_throw();
        archetype.show_labels =
            ComponentBatch::empty<rerun::components::ShowLabels>(Descriptor_show_labels)
                .value_or_throw();
        archetype.class_ids =
            ComponentBatch::empty<rerun::components::ClassId>(Descriptor_class_ids)
                .value_or_throw();
        return archetype;
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::Capsules3D>::serialize(
        const archetypes::Capsules3D& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(10);

        if (archetype.lengths.has_value()) {
            cells.push_back(archetype.lengths.value());
        }
        if (archetype.radii.has_value()) {
            cells.push_back(archetype.radii.value());
        }
        if (archetype.translations.has_value()) {
            cells.push_back(archetype.translations.value());
        }
        if (archetype.rotation_axis_angles.has_value()) {
            cells.push_back(archetype.rotation_axis_angles.value());
        }
        if (archetype.quaternions.has_value()) {
            cells.push_back(archetype.quaternions.value());
        }
        if (archetype.colors.has_value()) {
            cells.push_back(archetype.colors.value());
        }
        if (archetype.labels.has_value()) {
            cells.push_back(archetype.labels.value());
        }
        if (archetype.show_labels.has_value()) {
            cells.push_back(archetype.show_labels.value());
        }
        if (archetype.class_ids.has_value()) {
            cells.push_back(archetype.class_ids.value());
        }
        {
            auto indicator = Capsules3D::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
