use re_chunk::{Chunk, LatestAtQuery, RowId, Timeline};
use re_log_types::example_components::{MyColor, MyLabel, MyPoint};
use re_types_core::Component;

// ---

fn main() -> anyhow::Result<()> {
    let chunk = create_chunk()?;

    eprintln!("Data:\n{chunk}");

    let query = LatestAtQuery::new(Timeline::new_sequence("frame"), 4);

    // Find all relevant data for a query:
    // TODO
    let chunk = chunk.latest_at(&query, &MyPoint::descriptor());
    eprintln!("{:?} @ {query:?}:\n{chunk}", MyPoint::descriptor());

    // And then slice it as appropriate:
    let chunk = chunk
        .timeline_sliced(Timeline::log_time())
        .component_sliced(MyPoint::name()); // TODO: huhuh
    eprintln!("Sliced down to specific timeline and component:\n{chunk}");

    Ok(())
}

fn create_chunk() -> anyhow::Result<Chunk> {
    let mut chunk = Chunk::builder("my/entity".into())
        .with_component_batches(
            RowId::new(),
            [
                (Timeline::log_time(), 1000),
                (Timeline::new_sequence("frame"), 1),
            ],
            [
                &[MyPoint::new(1.0, 1.0), MyPoint::new(2.0, 2.0)] as _, //
            ],
        )
        .with_component_batches(
            RowId::new(),
            [
                (Timeline::log_time(), 1032),
                (Timeline::new_sequence("frame"), 3),
            ],
            [
                &[MyColor::from_rgb(1, 1, 1)] as _, //
                &[
                    MyLabel("a".into()),
                    MyLabel("b".into()),
                    MyLabel("c".into()),
                ] as _, //
            ],
        )
        .with_component_batches(
            RowId::new(),
            [
                (Timeline::log_time(), 1064),
                (Timeline::new_sequence("frame"), 5),
            ],
            [
                &[
                    MyPoint::new(3.0, 3.0),
                    MyPoint::new(4.0, 4.0),
                    MyPoint::new(5.0, 5.0),
                ] as _, //
            ],
        )
        .build()?;

    chunk.sort_if_unsorted();

    Ok(chunk)
}
