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
        // Extract an entry from the change event
        let entry = if let ChangeEvent::Added(Record::Value(RecordValue::Entry(entry))) = event {
            Some(entry)
        } else if let ChangeEvent::Replaced {
            from: Record::Value(RecordValue::Entry(entry_old)),
            to: Record::Value(RecordValue::Entry(entry_new)),
        } = event
        {
            // We are replacing an entry, so remove old one if existed
            self.data.remove(entry_old.entry());
            Some(entry_new)
        } else {
            None
        };

        let Some(entry) = entry else { return };
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::db::RecordEntry;

    use super::*;

    const ENTRY_PREFIX: &str = "2000-01-01 00:00";

    fn record(s: &str) -> Record {
        Record::Value(RecordValue::Entry(RecordEntry::new(
            1,
            Entry::parse(&format!("{ENTRY_PREFIX} {s}")).unwrap(),
        )))
    }
    fn assert_data(view: &QueryResultsView, want: Vec<&'static str>) {
        let got: Vec<_> = view
            .data
            .iter()
            .map(|v| v.to_string()[ENTRY_PREFIX.len() + 1..].to_string())
            .collect();
        let want: Vec<_> = want.iter().map(|v| v.to_string()).collect();
        assert_eq!(got, want);
    }

    #[test]
    fn update_add() {
        let mut view = QueryResultsView::default();
        view.update_query(
            Query::new("tag1").unwrap(),
            BTreeMap::default().iter(),
            &None,
        );

        // Matching entry
        view.update(&ChangeEvent::Added(record("00:02 tag1")), &None);
        view.update(&ChangeEvent::Added(record("00:01 tag1")), &None);
        view.update(&ChangeEvent::Added(record("00:03 tag2")), &None);
        assert_data(&view, vec!["00:01 tag1", "00:02 tag1"]);

        // Nothing found
        view.update_query(
            Query::new("tag3").unwrap(),
            BTreeMap::default().iter(),
            &None,
        );
        assert_data(&view, vec![]);
    }

    #[test]
    fn update_replace() {
        let mut view = QueryResultsView::default();
        view.update_query(
            Query::new("tag1").unwrap(),
            BTreeMap::default().iter(),
            &None,
        );
        let rec1 = record("00:00 tag1. Comment1");
        let rec2 = record("00:00 tag1. Comment2");
        view.update(&ChangeEvent::Added(rec1.clone()), &None);
        view.update(
            &ChangeEvent::Replaced {
                from: rec1,
                to: rec2,
            },
            &None,
        );
        assert_data(&view, vec!["00:00 tag1. Comment2"]);
    }
}
