use std::{ops::RangeBounds, sync::atomic::Ordering};

use arrow2::{
    array::{Array, Int64Array, ListArray, UInt64Array},
    datatypes::{DataType, TimeUnit},
};

use re_log::trace;
use re_log_types::{ComponentName, ObjPath as EntityPath, TimeInt, TimeRange, Timeline};

use crate::{
    ComponentBucket, ComponentTable, DataStore, IndexBucket, IndexBucketIndices, IndexTable,
    RowIndex, SecondaryIndex,
};

// --- Queries ---

/// A query a given time, for a given timeline.
///
/// Get the latest version of the data available at this time.
#[derive(Clone)]
pub struct LatestAtQuery {
    pub timeline: Timeline,
    pub at: TimeInt,
}

impl std::fmt::Debug for LatestAtQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "<latest at {} on {:?}>",
            self.timeline.typ().format(self.at),
            self.timeline.name(),
        ))
    }
}

impl LatestAtQuery {
    pub const fn new(timeline: Timeline, at: TimeInt) -> Self {
        Self { timeline, at }
    }
}

/// A query over a time range, for a given timeline.
///
/// Get all the data within this time interval, plus the latest one before the start of the
/// interval.
///
/// Motivation: all data is considered alive until the next logging to the same data path.
#[derive(Clone)]
pub struct RangeQuery {
    pub timeline: Timeline,
    pub range: TimeRange,
}

impl std::fmt::Debug for RangeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "<ranging from {} to {} (all inclusive) on {:?}>",
            self.timeline.typ().format(self.range.min),
            self.timeline.typ().format(self.range.max),
            self.timeline.name(),
        ))
    }
}

impl RangeQuery {
    pub const fn new(timeline: Timeline, range: TimeRange) -> Self {
        Self { timeline, range }
    }
}

// --- Data store ---

impl DataStore {
    /// Queries the datastore for the internal row indices of the specified `components`, as seen
    /// from the point of view of the so-called `primary` component.
    ///
    /// Returns an array of row indices on success, or `None` otherwise.
    /// Success is defined by one thing and thing only: whether a row index could be found for the
    /// `primary` component.
    /// The presence or absence of secondary components has no effect on the success criteria.
    ///
    /// * On success, the returned array is filled with the internal row index of each and every
    ///   component in `components`, or `None` if said component isn't available at that point
    ///   in time.
    ///
    /// To actually retrieve the data associated with these indices, see [`Self::get`].
    ///
    /// Follows a complete example of querying indices, fetching the associated data and finally
    /// turning it all into a `polars::DataFrame`:
    /// ```rust
    /// # use polars_core::prelude::*;
    /// # use arrow2::array::Array;
    /// # use re_log_types::{*, ObjPath as EntityPath};
    /// # use re_arrow_store::*;
    ///
    /// fn fetch_latest_components<const N: usize>(
    ///     store: &DataStore,
    ///     query: &LatestAtQuery,
    ///     ent_path: &EntityPath,
    ///     primary: ComponentName,
    ///     components: &[ComponentName; N],
    /// ) -> DataFrame {
    ///     let row_indices = store
    ///         .latest_at(query, ent_path, primary, components)
    ///         .unwrap_or([None; N]);
    ///     let results = store.get(components, &row_indices);
    ///
    ///     let df = {
    ///         let series: Vec<_> = components
    ///             .iter()
    ///             .zip(results)
    ///             .filter_map(|(component, col)| col.map(|col| (component, col)))
    ///             .map(|(&component, col)| Series::try_from((component.as_str(), col)).unwrap())
    ///             .collect();
    ///
    ///         DataFrame::new(series).unwrap()
    ///     };
    ///
    ///     df
    /// }
    /// ```
    //
    // TODO: visual doc of latest_at behavior/semantics
    //
    // TODO(cmc): expose query_dyn at some point, to fetch an unknown number of component indices,
    // at the cost of extra dynamic allocations.
    pub fn latest_at<const N: usize>(
        &self,
        query: &LatestAtQuery,
        ent_path: &EntityPath,
        primary: ComponentName,
        components: &[ComponentName; N],
    ) -> Option<[Option<RowIndex>; N]> {
        // TODO(cmc): kind & query_id need to somehow propagate through the span system.
        self.query_id.fetch_add(1, Ordering::Relaxed);

        let ent_path_hash = ent_path.hash();

        trace!(
            kind = "latest_at",
            id = self.query_id.load(Ordering::Relaxed),
            query = ?query,
            entity = %ent_path,
            %primary,
            ?components,
            "query started..."
        );

        if let Some(index) = self.indices.get(&(query.timeline, *ent_path_hash)) {
            if let row_indices @ Some(_) = index.latest_at(query.at, primary, components) {
                trace!(
                    kind = "latest_at",
                    query = ?query,
                    entity = %ent_path,
                    %primary,
                    ?components,
                    ?row_indices,
                    "row indices fetched"
                );
                return row_indices;
            }
        }

        trace!(
            kind = "latest_at",
            query = ?query,
            entity = %ent_path,
            %primary,
            ?components,
            "primary component not found"
        );

        None
    }

    // TODO
    // TODO: visual doc of range behavior/semantics
    pub fn range<'a, const N: usize>(
        &'a self,
        query: &RangeQuery,
        ent_path: &EntityPath,
        primary: ComponentName,
        components: [ComponentName; N],
    ) -> impl Iterator<Item = (TimeInt, [Option<RowIndex>; N])> + 'a {
        // TODO(cmc): kind & query_id need to somehow propagate through the span system.
        self.query_id.fetch_add(1, Ordering::Relaxed);

        let ent_path_hash = ent_path.hash();

        trace!(
            kind = "range",
            id = self.query_id.load(Ordering::Relaxed),
            query = ?query,
            entity = %ent_path,
            %primary,
            ?components,
            "query started..."
        );

        let index = self.indices.get(&(query.timeline, *ent_path_hash));

        // First off, get the latest state just before the start of the time range.
        let latest_time = query.range.min.as_i64().saturating_sub(1).into();
        let latest_row_indices =
            index.and_then(|index| index.latest_at(latest_time, primary, &components));
        trace!(
            kind = "range",
            id = self.query_id.load(Ordering::Relaxed),
            query = ?query,
            entity = %ent_path,
            %primary,
            ?components,
            ?latest_row_indices,
            "latest row indices fetched"
        );

        std::iter::once(latest_row_indices)
            .filter_map(move |latest| latest.map(|latest| (latest_time, latest)))
            .chain(
                index
                    .map(|index| index.range(query.range, primary, components))
                    .into_iter()
                    .flatten(),
            )
    }

    /// Retrieves the data associated with a list of `components` at the specified `indices`.
    ///
    /// If the associated data is found, it will be written into the returned array at the
    /// appropriate index, or `None` otherwise.
    ///
    /// `row_indices` takes a list of options so that one can easily re-use the results obtained
    /// from [`Self::query`].
    ///
    /// See [`Self::query`] for more information.
    pub fn get<const N: usize>(
        &self,
        components: &[ComponentName; N],
        row_indices: &[Option<RowIndex>; N],
    ) -> [Option<Box<dyn Array>>; N] {
        let mut results = [(); N].map(|_| None); // work around non-Copy const initialization limitations

        for (i, &component, row_idx) in components
            .iter()
            .zip(row_indices)
            .enumerate()
            .filter_map(|(i, (comp, row_idx))| row_idx.map(|row_idx| (i, comp, row_idx)))
        {
            let row = self
                .components
                .get(&component)
                .and_then(|table| table.get(row_idx));
            results[i] = row;
        }

        results
    }

    /// Force the sorting of all indices.
    pub fn sort_indices(&mut self) {
        for index in self.indices.values_mut() {
            index.sort_indices();
        }
    }

    /// Returns a read-only iterator over the raw index tables.
    ///
    /// Do _not_ use this to try and test the internal state of the datastore.
    pub fn iter_indices(
        &self,
    ) -> impl ExactSizeIterator<Item = ((Timeline, EntityPath), &IndexTable)> {
        self.indices.iter().map(|((timeline, _), table)| {
            ((*timeline, table.ent_path.clone() /* shallow */), table)
        })
    }
}

// --- Indices ---

impl IndexTable {
    /// Returns `None` iff no row index could be found for the `primary` component.
    pub fn latest_at<const N: usize>(
        &self,
        time: TimeInt,
        primary: ComponentName,
        components: &[ComponentName; N],
    ) -> Option<[Option<RowIndex>; N]> {
        let timeline = self.timeline;

        // The time we're looking for gives us an upper bound: all components must be indexed
        // in either this bucket _or any of those that come before_!
        //
        // That is because secondary indices allow for null values, which forces us to not only
        // walk backwards within an index bucket, but sometimes even walk backwards across
        // multiple index buckets within the same table!

        for (attempt, bucket) in self.range_buckets_rev(..=time).enumerate() {
            trace!(
                kind = "latest_at",
                timeline = %timeline.name(),
                time = timeline.typ().format(time),
                %primary,
                ?components,
                attempt,
                bucket_time_range = timeline.typ().format_range(bucket.indices.read().time_range),
                "found candidate bucket"
            );
            if let row_indices @ Some(_) = bucket.latest_at(time, primary, components) {
                return row_indices; // found at least the primary component!
            }
        }

        None // primary component not found
    }

    // TODO
    pub fn range<'a, const N: usize>(
        &'a self,
        time_range: TimeRange,
        primary: ComponentName,
        components: [ComponentName; N],
    ) -> impl Iterator<Item = (TimeInt, [Option<RowIndex>; N])> + 'a {
        let timeline = self.timeline;

        // TODO: explain lack of min clause
        self.range_buckets(..=time_range.max)
            .enumerate()
            .flat_map(move |(bucket_nr, bucket)| {
                trace!(
                    kind = "range",
                    bucket_nr,
                    bucket_time_range =
                        timeline.typ().format_range(bucket.indices.read().time_range),
                    timeline = %timeline.name(),
                    ?time_range,
                    %primary,
                    ?components,
                    "found bucket in range"
                );

                bucket.range(time_range, primary, components)
            })
    }

    /// Returns the index bucket whose time range covers the given `time`.
    pub fn find_bucket_mut(&mut self, time: TimeInt) -> &mut IndexBucket {
        // This cannot fail, `iter_bucket_mut` is guaranteed to always yield at least one bucket,
        // since index tables always spawn with a default bucket that covers [-∞;+∞].
        self.range_bucket_rev_mut(..=time).next().unwrap()
    }

    /// Returns an iterator that is guaranteed to yield at least one bucket, which is the bucket
    /// whose time range covers the start bound of the given `time_range`.
    ///
    /// It then continues yielding buckets until it runs out, in decreasing time range order.
    pub fn range_buckets(
        &self,
        time_range: impl RangeBounds<TimeInt>,
    ) -> impl Iterator<Item = &IndexBucket> {
        self.buckets.range(time_range).map(|(_, bucket)| bucket)
    }

    /// Returns an iterator that is guaranteed to yield at least one bucket, which is the bucket
    /// whose time range covers the end bound of the given `time_range`.
    ///
    /// It then continues yielding buckets until it runs out, in decreasing time range order.
    pub fn range_buckets_rev(
        &self,
        time_range: impl RangeBounds<TimeInt>,
    ) -> impl Iterator<Item = &IndexBucket> {
        self.buckets
            .range(time_range)
            .rev()
            .map(|(_, bucket)| bucket)
    }

    /// Returns an iterator that is guaranteed to yield at least one bucket, which is the bucket
    /// whose time range covers the end bound of the given `time_range`.
    ///
    /// It then continues yielding buckets until it runs out, in decreasing time range order.
    pub fn range_bucket_rev_mut(
        &mut self,
        time_range: impl RangeBounds<TimeInt>,
    ) -> impl Iterator<Item = &mut IndexBucket> {
        self.buckets
            .range_mut(time_range)
            .rev()
            .map(|(_, bucket)| bucket)
    }

    /// Force the sorting of all buckets.
    pub fn sort_indices(&self) {
        for bucket in self.buckets.values() {
            bucket.sort_indices();
        }
    }

    /// Returns a read-only iterator over the raw buckets.
    ///
    /// Do _not_ use this to try and test the internal state of the datastore.
    pub fn iter_buckets(&self) -> impl ExactSizeIterator<Item = &IndexBucket> {
        self.buckets.values()
    }
}

impl IndexBucket {
    /// Sort all indices by time.
    pub fn sort_indices(&self) {
        if self.indices.read().is_sorted {
            return; // early read-only exit
        }

        self.indices.write().sort();
    }

    /// Returns `None` iff no row index could be found for the `primary` component.
    pub fn latest_at<const N: usize>(
        &self,
        time: TimeInt,
        primary: ComponentName,
        components: &[ComponentName; N],
    ) -> Option<[Option<RowIndex>; N]> {
        self.sort_indices();

        let IndexBucketIndices {
            is_sorted: _,
            time_range: _,
            times,
            indices,
        } = &*self.indices.read();

        // Early-exit if this bucket is unaware of this component.
        let index = indices.get(&primary)?;

        trace!(
            kind = "latest_at",
            %primary,
            ?components,
            timeline = %self.timeline.name(),
            time = self.timeline.typ().format(time),
            "searching for primary & secondary row indices..."
        );

        // find the primary index's row.
        let primary_idx = times.partition_point(|t| *t <= time.as_i64()) as i64;

        // The partition point is always _beyond_ the index that we're looking for.
        // A partition point of 0 thus means that we're trying to query for data that lives
        // _before_ the beginning of time... there's nothing to be found there.
        if primary_idx == 0 {
            return None;
        }

        // The partition point is always _beyond_ the index that we're looking for; we need
        // to step back to find what we came for.
        let primary_idx = primary_idx - 1;
        trace!(
            kind = "latest_at",
            %primary,
            ?components,
            timeline = %self.timeline.name(),
            time = self.timeline.typ().format(time),
            %primary_idx,
            "found primary index",
        );

        // find the secondary indices' rows, and the associated row indices.
        let mut secondary_idx = primary_idx;
        while index[secondary_idx as usize].is_none() {
            secondary_idx -= 1;
            if secondary_idx < 0 {
                trace!(
                    kind = "latest_at",
                    %primary,
                    ?components,
                    timeline = %self.timeline.name(),
                    time = self.timeline.typ().format(time),
                    %primary_idx,
                    "no secondary index found",
                );
                return None;
            }
        }

        trace!(
            kind = "latest_at",
            %primary,
            ?components,
            timeline = %self.timeline.name(),
            time = self.timeline.typ().format(time),
            %primary_idx, %secondary_idx,
            "found secondary index",
        );
        debug_assert!(index[secondary_idx as usize].is_some());

        let mut row_indices = [None; N];
        for (i, component) in components.iter().enumerate() {
            if let Some(index) = indices.get(component) {
                if let Some(row_idx) = index[secondary_idx as usize] {
                    trace!(
                        kind = "latest_at",
                        %primary,
                        %component,
                        timeline = %self.timeline.name(),
                        time = self.timeline.typ().format(time),
                        %primary_idx, %secondary_idx, %row_idx,
                        "found row index",
                    );
                    row_indices[i] = Some(row_idx);
                }
            }
        }

        Some(row_indices)
    }

    // TODO
    pub fn range<'a, const N: usize>(
        &'a self,
        time_range: TimeRange,
        primary: ComponentName,
        components: [ComponentName; N],
    ) -> impl Iterator<Item = (TimeInt, [Option<RowIndex>; N])> + 'a {
        self.sort_indices();

        let IndexBucketIndices {
            is_sorted: _,
            time_range: bucket_time_range,
            times,
            indices,
        } = &*self.indices.read();

        let bucket_time_range = *bucket_time_range;

        let secondary_idx = 'search: {
            // Early-exit if this bucket is unaware of this component.
            let Some(index) = indices.get(&primary) else { break 'search None };

            trace!(
                kind = "range",
                bucket_time_range = self.timeline.typ().format_range(bucket_time_range),
                %primary,
                ?components,
                timeline = %self.timeline.name(),
                time_range = self.timeline.typ().format_range(time_range),
                "searching for primary & secondary row indices..."
            );

            // find the primary index's row.
            let primary_idx = times.partition_point(|t| *t < time_range.min.as_i64()) as i64;

            // TODO: explain
            if primary_idx >= times.len() as i64 {
                return itertools::Either::Right(std::iter::empty());
            }

            trace!(
                kind = "range",
                bucket_time_range = self.timeline.typ().format_range(bucket_time_range),
                %primary,
                ?components,
                timeline = %self.timeline.name(),
                time_range = self.timeline.typ().format_range(time_range),
                %primary_idx,
                "found primary index",
            );

            // find the secondary indices' rows, and the associated row indices.
            let mut secondary_idx = primary_idx;
            while index[secondary_idx as usize].is_none() {
                secondary_idx -= 1;
                if secondary_idx < 0 {
                    trace!(
                        kind = "range",
                        bucket_time_range = self.timeline.typ().format_range(bucket_time_range),
                        %primary,
                        ?components,
                        timeline = %self.timeline.name(),
                        time_range = self.timeline.typ().format_range(time_range),
                        %primary_idx,
                        "no secondary index found",
                    );
                    break 'search None;
                }
            }

            trace!(
                kind = "range",
                bucket_time_range = self.timeline.typ().format_range(bucket_time_range),
                %primary,
                ?components,
                timeline = %self.timeline.name(),
                time_range = self.timeline.typ().format_range(time_range),
                %primary_idx, %secondary_idx,
                "found secondary index",
            );
            debug_assert!(index[secondary_idx as usize].is_some());

            let secondary_time = times[secondary_idx as usize];
            time_range
                .contains(secondary_time.into())
                .then_some(secondary_idx)
        };

        let times = times.clone(); // TODO: gotta arc it
        let indices = indices.clone(); // TODO: what's the shallowness like in there

        let Some(secondary_idx) = secondary_idx else {
            return itertools::Either::Right(std::iter::empty());
        };

        // TODO: explaaaaaaaaaaaaaaaaain
        let row_indices = times
            .into_iter()
            .skip(secondary_idx as usize)
            .filter(move |time| time_range.contains((*time).into()))
            .enumerate()
            .filter_map(move |(offset, time)| {
                let secondary_idx = secondary_idx as usize + offset;

                // TODO: explain
                indices
                    .get(&primary)
                    .and_then(|index| index.get(secondary_idx).copied())??;

                let mut row_indices = [None; N];
                for (i, component) in components.iter().enumerate() {
                    if let Some(index) = indices.get(component) {
                        if let Some(row_idx) = index[secondary_idx] {
                            trace!(
                                kind = "range",
                                bucket_time_range =
                                    self.timeline.typ().format_range(bucket_time_range),
                                %primary,
                                %component,
                                timeline = %self.timeline.name(),
                                time_range = self.timeline.typ().format_range(time_range),
                                %secondary_idx, %row_idx,
                                "found row index",
                            );
                            row_indices[i] = Some(row_idx);
                        }
                    }
                }

                Some((time.into(), row_indices))
            });

        itertools::Either::Left(row_indices)
    }

    /// Whether the indices in this `IndexBucket` are sorted
    pub fn is_sorted(&self) -> bool {
        self.indices.read().is_sorted
    }

    /// Returns an (name, [`Int64Array`]) with a logical type matching the timeline.
    pub fn times(&self) -> (String, Int64Array) {
        let times = Int64Array::from_vec(self.indices.read().times.clone());
        let logical_type = match self.timeline.typ() {
            re_log_types::TimeType::Time => DataType::Timestamp(TimeUnit::Nanosecond, None),
            re_log_types::TimeType::Sequence => DataType::Int64,
        };
        (self.timeline.name().to_string(), times.to(logical_type))
    }

    /// Returns a Vec each of (name, array) for each index in the bucket
    pub fn named_indices(&self) -> (Vec<ComponentName>, Vec<UInt64Array>) {
        self.indices
            .read()
            .indices
            .iter()
            .map(|(name, index)| {
                (
                    name,
                    UInt64Array::from(
                        index
                            .iter()
                            .map(|row_idx| row_idx.map(|row_idx| row_idx.as_u64()))
                            .collect::<Vec<_>>(),
                    ),
                )
            })
            .unzip()
    }
}

impl IndexBucketIndices {
    pub fn sort(&mut self) {
        let Self {
            is_sorted,
            time_range: _,
            times,
            indices,
        } = self;

        if *is_sorted {
            return;
        }

        let swaps = {
            let mut swaps = (0..times.len()).collect::<Vec<_>>();
            swaps.sort_by_key(|&i| &times[i]);
            swaps
                .iter()
                .copied()
                .enumerate()
                .map(|(to, from)| (from, to))
                .collect::<Vec<_>>()
        };

        // Yep, the reshuffle implementation is very dumb and very slow :)
        // TODO(#442): re_datastore: implement efficient shuffling on the read path.

        // shuffle time index back into a sorted state
        {
            let source = times.clone();
            for (from, to) in swaps.iter().copied() {
                times[to] = source[from];
            }
        }

        fn reshuffle_index(index: &mut SecondaryIndex, swaps: &[(usize, usize)]) {
            // shuffle data
            {
                let source = index.clone();
                for (from, to) in swaps.iter().copied() {
                    index[to] = source[from];
                }
            }
        }

        // shuffle component indices back into a sorted state
        for index in indices.values_mut() {
            reshuffle_index(index, &swaps);
        }

        *is_sorted = true;
    }
}

// --- Components ---

impl ComponentTable {
    pub fn get(&self, row_idx: RowIndex) -> Option<Box<dyn Array>> {
        let mut bucket_nr = self
            .buckets
            .partition_point(|bucket| row_idx.as_u64() >= bucket.row_offset);

        // The partition point will give us the index of the first bucket that has a row offset
        // strictly greater than the row index we're looking for, therefore we need to take a
        // step back to find what we're looking for.
        //
        // Since component tables always spawn with a default bucket at offset 0, the smallest
        // partition point that can ever be returned is one, thus this operation is overflow-safe.
        debug_assert!(bucket_nr > 0);
        bucket_nr -= 1;

        if let Some(bucket) = self.buckets.get(bucket_nr) {
            trace!(
                kind = "get",
                component = self.name.as_str(),
                %row_idx,
                bucket_nr,
                %bucket.row_offset,
                "fetching component data"
            );
            Some(bucket.get(row_idx))
        } else {
            trace!(
                kind = "get",
                component = self.name.as_str(),
                %row_idx,
                bucket_nr,
                "row index is out of bounds"
            );
            None
        }
    }

    /// Returns an iterator over the `ComponentBucket` in this table
    #[allow(dead_code)]
    pub fn iter_buckets(&self) -> impl ExactSizeIterator<Item = &ComponentBucket> {
        self.buckets.iter()
    }
}

impl ComponentBucket {
    /// Returns the name of the component stored in this bucket.
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns a shallow clone of the row data present at the given `row_idx`.
    pub fn get(&self, row_idx: RowIndex) -> Box<dyn Array> {
        let row_idx = row_idx.as_u64() - self.row_offset;
        // This has to be safe to unwrap, otherwise it would never have made it past insertion.
        if self.archived {
            debug_assert_eq!(self.chunks.len(), 1);
            self.chunks[0]
                .as_any()
                .downcast_ref::<ListArray<i32>>()
                .unwrap()
                .value(row_idx as _)
        } else {
            self.chunks[row_idx as usize]
                .as_any()
                .downcast_ref::<ListArray<i32>>()
                .unwrap()
                .value(0)
        }
    }

    /// Returns a shallow clone of all the chunks in this bucket.
    #[allow(dead_code)]
    pub fn data(&self) -> Vec<Box<dyn Array>> {
        self.chunks.clone() // shallow
    }

    /// Return an iterator over the time ranges in this bucket.
    #[allow(dead_code)]
    pub fn iter_time_ranges(&self) -> impl Iterator<Item = (&Timeline, &TimeRange)> {
        self.time_ranges.iter()
    }
}
