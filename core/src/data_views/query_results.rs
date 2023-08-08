use std::collections::{btree_map::Iter, BTreeSet};

use crate::{
    date_time::datetime::DateTimeRange,
    db::{ChangeEvent, Query, Record, ViewUpdate},
};

#[derive(Default)]
pub struct QueryResultsView {
    data: BTreeSet<Record>,
    query: Query,
}

impl QueryResultsView {
    pub fn update(
        &mut self,
        event: &ChangeEvent,
        on_view_update: &Option<Box<dyn Fn(ViewUpdate)>>,
    ) {
        // Extract an entry from the change event
        let record = if let ChangeEvent::Added(record) = event {
            Some(record)
        } else if let ChangeEvent::Replaced {
            from: record_old,
            to: record_new,
        } = event
        {
            // We are replacing an entry, so remove old one if existed
            self.data.remove(record_old);
            Some(record_new)
        } else {
            None
        };

        let Some(record) = record else { return };
        if record.is_deleted_record() {
            if let Some(update) = on_view_update {
                update(ViewUpdate::QueryResults);
            }
            return;
        }
        if !self.query.matches(record) {
            return; // Not a relevant entry for the query
        }
        self.data.insert(record.clone());
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
            if record.is_deleted_record() {
                continue;
            }
            if !self.query.matches(record) {
                continue;
            }
            results.insert(record.clone());
        }
        let updated = self.data != results;
        self.data = results;
        // Call an update only if query results got changed
        if let (Some(update), true) = (on_view_update, updated) {
            update(ViewUpdate::QueryResults);
        }
    }

    pub fn data(&self) -> &BTreeSet<Record> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::record::Entry;

    use super::*;

    const ENTRY_PREFIX: &str = "2000-01-01 00:00";

    fn record(s: &str) -> Record {
        Record::Entry(Entry::parse(&format!("{ENTRY_PREFIX} {s}")).unwrap())
    }

    fn assert_data(view: &QueryResultsView, want: Vec<&'static str>) {
        let got: Vec<_> = view
            .data
            .iter()
            .map(|v| v.to_string(true, true)[ENTRY_PREFIX.len() + 1..].to_string())
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

    #[test]
    fn update_delete() {
        let mut view = QueryResultsView::default();
        view.update_query(
            Query::new("tag1").unwrap(),
            BTreeMap::default().iter(),
            &None,
        );
        let rec1 = record("00:00 tag1");
        view.update(&ChangeEvent::Added(rec1.clone()), &None);
        let rec2 = Record::parse(&rec1.to_deleted_string()).unwrap();
        view.update(
            &ChangeEvent::Replaced {
                from: rec1.clone(),
                to: rec2.clone(),
            },
            &None,
        );
        assert_data(&view, vec![]);
        // Updating query to empty doesn't return deleted entries
        let all = BTreeMap::from([(*rec1.date_range(), rec1), (*rec2.date_range(), rec2)]);
        view.update_query(Query::new("").unwrap(), all.iter(), &None);
        assert_data(&view, vec![]);
    }
}
