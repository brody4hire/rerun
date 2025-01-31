// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/boxes3d.fbs".

#include "boxes3d.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    Boxes3D Boxes3D::clear_fields() {
        auto archetype = Boxes3D();
        archetype.half_sizes =
            ComponentBatch::empty<rerun::components::HalfSize3D>(Descriptor_half_sizes)
                .value_or_throw();
        archetype.centers =
            ComponentBatch::empty<rerun::components::PoseTranslation3D>(Descriptor_centers)
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
        archetype.radii =
            ComponentBatch::empty<rerun::components::Radius>(Descriptor_radii).value_or_throw();
        archetype.fill_mode =
            ComponentBatch::empty<rerun::components::FillMode>(Descriptor_fill_mode)
                .value_or_throw();
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

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::Boxes3D>::serialize(
        const archetypes::Boxes3D& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(11);

        if (archetype.half_sizes.has_value()) {
            cells.push_back(archetype.half_sizes.value());
        }
        if (archetype.centers.has_value()) {
            cells.push_back(archetype.centers.value());
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
        if (archetype.radii.has_value()) {
            cells.push_back(archetype.radii.value());
        }
        if (archetype.fill_mode.has_value()) {
            cells.push_back(archetype.fill_mode.value());
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
            auto indicator = Boxes3D::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
