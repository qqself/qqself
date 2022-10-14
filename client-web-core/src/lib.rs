use qqself_core::datetime::{Date, DatePeriod, DateTime, DateTimeRange, TimeDuration};
use qqself_core::db::Query;
use qqself_core::parser::Parser;
use qqself_core::{db, encryption};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod utils;

/*

- Syncer
 - 1. Is WebWorker a way to unblock the main event-loop when we do encryption?
 - 2. Can we call WebAssembly from WebWorker?
 - Get new data from API
 - Upload new data to API

- DB
  - Create a new one with ProgressViews subscribed with optional cached ProgressViewState
  - Subscribe to changes
  - Bulk load from Cache
  - Syncer:
    - Get new data from API
    - Upload new data from API, first persist it in a cache
  - Get new data from API with disabled subscribers
  - Add new Record
- ProgressViews callbacks
  - One callback per each ProgressView
 */
