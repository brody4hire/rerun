// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/transform3d.fbs".

#include "transform3d.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    Transform3D Transform3D::clear_fields() {
        auto archetype = Transform3D();
        archetype.translation =
            ComponentBatch::empty<rerun::components::Translation3D>(Descriptor_translation)
                .value_or_throw();
        archetype.rotation_axis_angle = ComponentBatch::empty<rerun::components::RotationAxisAngle>(
                                            Descriptor_rotation_axis_angle
        )
                                            .value_or_throw();
        archetype.quaternion =
            ComponentBatch::empty<rerun::components::RotationQuat>(Descriptor_quaternion)
                .value_or_throw();
        archetype.scale =
            ComponentBatch::empty<rerun::components::Scale3D>(Descriptor_scale).value_or_throw();
        archetype.mat3x3 =
            ComponentBatch::empty<rerun::components::TransformMat3x3>(Descriptor_mat3x3)
                .value_or_throw();
        archetype.relation =
            ComponentBatch::empty<rerun::components::TransformRelation>(Descriptor_relation)
                .value_or_throw();
        archetype.axis_length =
            ComponentBatch::empty<rerun::components::AxisLength>(Descriptor_axis_length)
                .value_or_throw();
        return archetype;
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::Transform3D>::serialize(
        const archetypes::Transform3D& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(8);

        if (archetype.translation.has_value()) {
            cells.push_back(archetype.translation.value());
        }
        if (archetype.rotation_axis_angle.has_value()) {
            cells.push_back(archetype.rotation_axis_angle.value());
        }
        if (archetype.quaternion.has_value()) {
            cells.push_back(archetype.quaternion.value());
        }
        if (archetype.scale.has_value()) {
            cells.push_back(archetype.scale.value());
        }
        if (archetype.mat3x3.has_value()) {
            cells.push_back(archetype.mat3x3.value());
        }
        if (archetype.relation.has_value()) {
            cells.push_back(archetype.relation.value());
        }
        if (archetype.axis_length.has_value()) {
            cells.push_back(archetype.axis_length.value());
        }
        {
            auto indicator = Transform3D::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
