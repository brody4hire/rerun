// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/testing/archetypes/fuzzy.fbs".

#pragma once

#include "../components/affix_fuzzer1.hpp"
#include "../components/affix_fuzzer10.hpp"
#include "../components/affix_fuzzer11.hpp"
#include "../components/affix_fuzzer12.hpp"
#include "../components/affix_fuzzer13.hpp"
#include "../components/affix_fuzzer14.hpp"
#include "../components/affix_fuzzer15.hpp"
#include "../components/affix_fuzzer16.hpp"
#include "../components/affix_fuzzer17.hpp"
#include "../components/affix_fuzzer18.hpp"
#include "../components/affix_fuzzer2.hpp"
#include "../components/affix_fuzzer3.hpp"
#include "../components/affix_fuzzer4.hpp"
#include "../components/affix_fuzzer5.hpp"
#include "../components/affix_fuzzer6.hpp"
#include "../components/affix_fuzzer7.hpp"
#include "../components/affix_fuzzer8.hpp"
#include "../components/affix_fuzzer9.hpp"

#include <cstdint>
#include <optional>
#include <rerun/collection.hpp>
#include <rerun/compiler_utils.hpp>
#include <rerun/component_batch.hpp>
#include <rerun/indicator_component.hpp>
#include <rerun/result.hpp>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    struct AffixFuzzer4 {
        std::optional<Collection<rerun::components::AffixFuzzer1>> fuzz2101;

        std::optional<Collection<rerun::components::AffixFuzzer2>> fuzz2102;

        std::optional<Collection<rerun::components::AffixFuzzer3>> fuzz2103;

        std::optional<Collection<rerun::components::AffixFuzzer4>> fuzz2104;

        std::optional<Collection<rerun::components::AffixFuzzer5>> fuzz2105;

        std::optional<Collection<rerun::components::AffixFuzzer6>> fuzz2106;

        std::optional<Collection<rerun::components::AffixFuzzer7>> fuzz2107;

        std::optional<Collection<rerun::components::AffixFuzzer8>> fuzz2108;

        std::optional<Collection<rerun::components::AffixFuzzer9>> fuzz2109;

        std::optional<Collection<rerun::components::AffixFuzzer10>> fuzz2110;

        std::optional<Collection<rerun::components::AffixFuzzer11>> fuzz2111;

        std::optional<Collection<rerun::components::AffixFuzzer12>> fuzz2112;

        std::optional<Collection<rerun::components::AffixFuzzer13>> fuzz2113;

        std::optional<Collection<rerun::components::AffixFuzzer14>> fuzz2114;

        std::optional<Collection<rerun::components::AffixFuzzer15>> fuzz2115;

        std::optional<Collection<rerun::components::AffixFuzzer16>> fuzz2116;

        std::optional<Collection<rerun::components::AffixFuzzer17>> fuzz2117;

        std::optional<Collection<rerun::components::AffixFuzzer18>> fuzz2118;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.testing.components.AffixFuzzer4Indicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        static constexpr const char ArchetypeName[] = "rerun.testing.archetypes.AffixFuzzer4";

      public:
        AffixFuzzer4() = default;
        AffixFuzzer4(AffixFuzzer4&& other) = default;
        AffixFuzzer4(const AffixFuzzer4& other) = default;
        AffixFuzzer4& operator=(const AffixFuzzer4& other) = default;
        AffixFuzzer4& operator=(AffixFuzzer4&& other) = default;

        AffixFuzzer4 with_fuzz2101(Collection<rerun::components::AffixFuzzer1> _fuzz2101) && {
            fuzz2101 = std::move(_fuzz2101);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2102(Collection<rerun::components::AffixFuzzer2> _fuzz2102) && {
            fuzz2102 = std::move(_fuzz2102);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2103(Collection<rerun::components::AffixFuzzer3> _fuzz2103) && {
            fuzz2103 = std::move(_fuzz2103);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2104(Collection<rerun::components::AffixFuzzer4> _fuzz2104) && {
            fuzz2104 = std::move(_fuzz2104);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2105(Collection<rerun::components::AffixFuzzer5> _fuzz2105) && {
            fuzz2105 = std::move(_fuzz2105);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2106(Collection<rerun::components::AffixFuzzer6> _fuzz2106) && {
            fuzz2106 = std::move(_fuzz2106);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2107(Collection<rerun::components::AffixFuzzer7> _fuzz2107) && {
            fuzz2107 = std::move(_fuzz2107);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2108(Collection<rerun::components::AffixFuzzer8> _fuzz2108) && {
            fuzz2108 = std::move(_fuzz2108);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2109(Collection<rerun::components::AffixFuzzer9> _fuzz2109) && {
            fuzz2109 = std::move(_fuzz2109);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2110(Collection<rerun::components::AffixFuzzer10> _fuzz2110) && {
            fuzz2110 = std::move(_fuzz2110);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2111(Collection<rerun::components::AffixFuzzer11> _fuzz2111) && {
            fuzz2111 = std::move(_fuzz2111);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2112(Collection<rerun::components::AffixFuzzer12> _fuzz2112) && {
            fuzz2112 = std::move(_fuzz2112);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2113(Collection<rerun::components::AffixFuzzer13> _fuzz2113) && {
            fuzz2113 = std::move(_fuzz2113);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2114(Collection<rerun::components::AffixFuzzer14> _fuzz2114) && {
            fuzz2114 = std::move(_fuzz2114);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2115(Collection<rerun::components::AffixFuzzer15> _fuzz2115) && {
            fuzz2115 = std::move(_fuzz2115);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2116(Collection<rerun::components::AffixFuzzer16> _fuzz2116) && {
            fuzz2116 = std::move(_fuzz2116);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2117(Collection<rerun::components::AffixFuzzer17> _fuzz2117) && {
            fuzz2117 = std::move(_fuzz2117);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        AffixFuzzer4 with_fuzz2118(Collection<rerun::components::AffixFuzzer18> _fuzz2118) && {
            fuzz2118 = std::move(_fuzz2118);
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
    struct AsComponents<archetypes::AffixFuzzer4> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const archetypes::AffixFuzzer4& archetype
        );
    };
} // namespace rerun
