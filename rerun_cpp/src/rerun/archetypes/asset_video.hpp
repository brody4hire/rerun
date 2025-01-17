// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/asset_video.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/blob.hpp"
#include "../components/media_type.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <chrono>
#include <cstdint>
#include <filesystem>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A video binary.
    ///
    /// Only MP4 containers with AV1 are generally supported,
    /// though the web viewer supports more video codecs, depending on browser.
    ///
    /// See <https://rerun.io/docs/reference/video> for details of what is and isn't supported.
    ///
    /// In order to display a video, you also need to log a `archetypes::VideoFrameReference` for each frame.
    ///
    /// ## Examples
    ///
    /// ### Video with automatically determined frames
    /// ![image](https://static.rerun.io/video_manual_frames/320a44e1e06b8b3a3161ecbbeae3e04d1ccb9589/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <iostream>
    ///
    /// using namespace std::chrono_literals;
    ///
    /// int main(int argc, char* argv[]) {
    ///     if (argc <2) {
    ///         // TODO(#7354): Only mp4 is supported for now.
    ///         std::cerr <<"Usage: " <<argv[0] <<" <path_to_video.[mp4]>" <<std::endl;
    ///         return 1;
    ///     }
    ///
    ///     const auto path = argv[1];
    ///
    ///     const auto rec = rerun::RecordingStream("rerun_example_asset_video_auto_frames");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Log video asset which is referred to by frame references.
    ///     auto video_asset = rerun::AssetVideo::from_file(path).value_or_throw();
    ///     rec.log_static("video", video_asset);
    ///
    ///     // Send automatically determined video frame timestamps.
    ///     std::vector<std::chrono::nanoseconds> frame_timestamps_ns =
    ///         video_asset.read_frame_timestamps_ns().value_or_throw();
    ///     // Note timeline values don't have to be the same as the video timestamps.
    ///     auto time_column =
    ///         rerun::TimeColumn::from_times("video_time", rerun::borrow(frame_timestamps_ns));
    ///
    ///     std::vector<rerun::components::VideoTimestamp> video_timestamps(frame_timestamps_ns.size());
    ///     for (size_t i = 0; i <frame_timestamps_ns.size(); i++) {
    ///         video_timestamps[i] = rerun::components::VideoTimestamp(frame_timestamps_ns[i]);
    ///     }
    ///     auto video_frame_reference_indicators =
    ///         rerun::ComponentColumn::from_indicators<rerun::VideoFrameReference>(
    ///             static_cast<uint32_t>(video_timestamps.size())
    ///         );
    ///
    ///     rec.send_columns(
    ///         "video",
    ///         time_column,
    ///         {
    ///             video_frame_reference_indicators.value_or_throw(),
    ///             rerun::ComponentColumn::from_loggable(rerun::borrow(video_timestamps)).value_or_throw(),
    ///         }
    ///     );
    /// }
    /// ```
    ///
    /// ### Demonstrates manual use of video frame references
    /// ![image](https://static.rerun.io/video_manual_frames/9f41c00f84a98cc3f26875fba7c1d2fa2bad7151/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <iostream>
    ///
    /// using namespace std::chrono_literals;
    ///
    /// int main(int argc, char* argv[]) {
    ///     if (argc <2) {
    ///         // TODO(#7354): Only mp4 is supported for now.
    ///         std::cerr <<"Usage: " <<argv[0] <<" <path_to_video.[mp4]>" <<std::endl;
    ///         return 1;
    ///     }
    ///
    ///     const auto path = argv[1];
    ///
    ///     const auto rec = rerun::RecordingStream("rerun_example_asset_video_manual_frames");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Log video asset which is referred to by frame references.
    ///     rec.log_static("video_asset", rerun::AssetVideo::from_file(path).value_or_throw());
    ///
    ///     // Create two entities, showing the same video frozen at different times.
    ///     rec.log("frame_1s", rerun::VideoFrameReference(1.0s).with_video_reference("video_asset"));
    ///     rec.log("frame_2s", rerun::VideoFrameReference(2.0s).with_video_reference("video_asset"));
    ///
    ///     // TODO(#5520): log blueprint once supported
    /// }
    /// ```
    struct AssetVideo {
        /// The asset's bytes.
        rerun::components::Blob blob;

        /// The Media Type of the asset.
        ///
        /// Supported values:
        /// * `video/mp4`
        ///
        /// If omitted, the viewer will try to guess from the data blob.
        /// If it cannot guess, it won't be able to render the asset.
        std::optional<rerun::components::MediaType> media_type;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.AssetVideoIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        static constexpr const char ArchetypeName[] = "rerun.archetypes.AssetVideo";

      public: // START of extensions from asset_video_ext.cpp:
        /// Creates a new `AssetVideo` from the file contents at `path`.
        ///
        /// The `MediaType` will be guessed from the file extension.
        ///
        /// If no `MediaType` can be guessed at the moment, the Rerun Viewer will try to guess one
        /// from the data at render-time. If it can't, rendering will fail with an error.
        static Result<AssetVideo> from_file(const std::filesystem::path& path);

        /// Creates a new `AssetVideo` from the given `bytes`.
        ///
        /// If no `MediaType` is specified, the Rerun Viewer will try to guess one from the data
        /// at render-time. If it can't, rendering will fail with an error.
        static AssetVideo from_bytes(
            rerun::Collection<uint8_t> bytes,
            std::optional<rerun::components::MediaType> media_type = {}
        ) {
            // TODO(jan): we could try and guess using magic bytes here, like rust does.
            AssetVideo asset = AssetVideo(std::move(bytes));
            asset.media_type = media_type;
            return asset;
        }

        /// Determines the presentation timestamps of all frames inside the video.
        ///
        /// Returned timestamps are in nanoseconds since start and are guaranteed to be monotonically increasing.
        Result<std::vector<std::chrono::nanoseconds>> read_frame_timestamps_ns() const;

        // END of extensions from asset_video_ext.cpp, start of generated code:

      public:
        AssetVideo() = default;
        AssetVideo(AssetVideo&& other) = default;
        AssetVideo(const AssetVideo& other) = default;
        AssetVideo& operator=(const AssetVideo& other) = default;
        AssetVideo& operator=(AssetVideo&& other) = default;

        explicit AssetVideo(rerun::components::Blob _blob) : blob(std::move(_blob)) {}

        /// The Media Type of the asset.
        ///
        /// Supported values:
        /// * `video/mp4`
        ///
        /// If omitted, the viewer will try to guess from the data blob.
        /// If it cannot guess, it won't be able to render the asset.
        AssetVideo with_media_type(rerun::components::MediaType _media_type) && {
            media_type = std::move(_media_type);
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
    struct AsComponents<archetypes::AssetVideo> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(const archetypes::AssetVideo& archetype
        );
    };
} // namespace rerun
