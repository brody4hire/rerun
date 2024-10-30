use rerun::{
    external::arrow2, ChunkStore, ChunkStoreConfig, Component, ComponentDescriptor, Loggable,
    VersionPolicy,
};

#[derive(Debug, Clone, Copy)]
struct CustomPosition3D(rerun::components::Position3D);

impl rerun::SizeBytes for CustomPosition3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        0
    }
}

impl Loggable for CustomPosition3D {
    #[inline]
    fn arrow2_datatype() -> arrow2::datatypes::DataType {
        rerun::components::Position3D::arrow2_datatype()
    }

    #[inline]
    fn to_arrow2_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> rerun::SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: 'a,
    {
        rerun::components::Position3D::to_arrow2_opt(
            data.into_iter().map(|opt| opt.map(Into::into).map(|c| c.0)),
        )
    }
}

impl Component for CustomPosition3D {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("user.CustomArchetype".into()),
            archetype_field_name: Some("user.CustomArchetypeField".into()),
            component_name: "user.CustomPosition3D".into(),
        }
    }
}

#[allow(clippy::unwrap_used)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const APP_ID: &str = "rerun_example_descriptors_custom_component_vanilla";

    let rec = rerun::RecordingStreamBuilder::new(APP_ID).spawn()?;

    let position = CustomPosition3D(rerun::components::Position3D::new(1.0, 2.0, 3.0));
    rec.log_component_batches("data", true, [&position as &dyn rerun::ComponentBatch])?;

    // When this snippet runs through the snippet comparison machinery, this environment variable
    // will point to the output RRD.
    // We can thus load this RRD to check that the proper tags were indeed forwarded.
    //
    // Python and C++ are indirectly checked by the snippet comparison tool itself.
    if let Ok(path_to_rrd) = std::env::var("_RERUN_TEST_FORCE_SAVE") {
        rec.flush_blocking();

        let stores = ChunkStore::from_rrd_filepath(
            &ChunkStoreConfig::ALL_DISABLED,
            path_to_rrd,
            VersionPolicy::Warn,
        )?;
        assert_eq!(1, stores.len());

        let store = stores.into_values().next().unwrap();
        let chunks = store.iter_chunks().collect::<Vec<_>>();
        assert_eq!(1, chunks.len());

        let chunk = chunks.into_iter().next().unwrap();

        let mut descriptors = chunk
            .components()
            .values()
            .flat_map(|per_desc| per_desc.keys())
            .cloned()
            .collect::<Vec<_>>();
        descriptors.sort();

        let expected = vec![CustomPosition3D::descriptor()];

        similar_asserts::assert_eq!(expected, descriptors);
    }

    Ok(())
}
