use std::collections::{btree_map::Iter, BTreeSet};

use crate::{
    date_time::datetime::DateTimeRange,
    db::{ChangeEvent, Query, Record, RecordValue, ViewUpdate},
    record::Entry,
};

#[derive(Default)]
pub struct QueryResultsView {
    data: BTreeSet<Entry>,
    query: Query,
}

impl QueryResultsView {
    pub fn update(
        &mut self,
        event: &ChangeEvent,
        on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>,
    ) {
        let ChangeEvent::Added(Record::Value(RecordValue::Entry(entry))) = event else { return };

        if !self.query.matches(entry.entry()) {
            return; // Not a relevant entry for the query
        }

        self.data.insert(entry.entry().clone());
        if let Some(update) = on_view_update {
            update(ViewUpdate::QueryResults);
        }
    }

    pub fn update_query(
        &mut self,
        query: Query,
        all: Iter<DateTimeRange, Record>,
        on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>,
    ) {
        self.query = query;
        // Query got updated, we need to reiterate all the entries
        // TODO There are better ways than recreating results from the scratch. In some cases query results are
        //      similar to the previous one and updating it should be much faster
        let mut results = BTreeSet::default();
        for (_, record) in all.clone() {
            let Record::Value(RecordValue::Entry(entry)) = record else { continue; };
            if !self.query.matches(entry.entry()) {
                continue;
            }
            results.insert(entry.entry().clone());
        }
        let updated = self.data != results;
        self.data = results;
        // Call an update only if query results got changed
        if let (Some(update), true) = (on_view_update, updated) {
            update(ViewUpdate::QueryResults);
        }
    }

    pub fn data(&self) -> &BTreeSet<Entry> {
        &self.data
    }
}
