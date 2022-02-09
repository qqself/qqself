use parser::{ParseError, Parser};
use record::{Entry, Goal, Record, Tag};

// Parsed collection of all active entries and goals
pub struct DB {
    entries: Vec<Entry>,
    goals: Vec<Goal>,
}

// To query entries filtered by certain conditions
#[derive(PartialEq)]
pub struct Query {
    pub query: Vec<Tag>,
    pub filter: Option<FilterDate>,
}

impl Default for Query {
    fn default() -> Self {
        Query {
            query: vec![],
            filter: None,
        }
    }
}

impl Query {
    fn new(query: &str, filter: Option<FilterDate>) -> Result<Query, ParseError> {
        let mut parser = Parser::new(&query);
        let query = parser.parse_query()?;
        Ok(Query { query, filter })
    }

    fn matches(&self, entry: &Entry) -> bool {
        false
    }
}

// Query results
pub struct QueryResults {
    entries: Vec<Entry>,
}

// Query execution error
pub enum QueryError {
    BadQuery(String),
}

// Entries filter by date
#[derive(PartialEq)]
pub struct FilterDate {
    pub time_from: usize,
    pub time_to: usize,
}

// Goal progress stats
pub struct GoalProgress {
    pub name: String,
    pub completion: usize,
    pub minutes_actual: usize,
    pub minutes_planned: usize,
}

impl DB {
    pub fn new() -> Self {
        DB {
            entries: vec![],
            goals: vec![],
        }
    }

    // Add new record to database
    pub fn add(&mut self, record: Record) {
        match record {
            Record::Entry(entry) => self.entries.push(entry),
            Record::Goal(goal) => self.goals.push(goal),
        }
    }

    pub fn query(&self, query: Query) -> Result<QueryResults, QueryError> {
        let mut entries: Vec<Entry> = Vec::new();
        for entry in &self.entries {
            if query.matches(entry) {
                entries.push(entry.clone());
            }
        }
        Ok(QueryResults { entries })
    }

    pub fn goal_progress(&self, filter: FilterDate) -> Vec<GoalProgress> {
        vec![]
    }
}
