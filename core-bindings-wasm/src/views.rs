use std::cell::RefCell;

use qqself_core::{
    data_views::skills::SkillsNotification,
    date_time::datetime::DateDay,
    db::{Notification, Query, Record, ViewUpdate, DB},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::util::error;

#[wasm_bindgen]
pub struct Views {
    db: RefCell<DB>,
}

#[wasm_bindgen]
pub struct UiRecord {
    record: Record,
}

#[wasm_bindgen]
impl UiRecord {
    pub fn to_string(&self, include_date: bool, include_entry_tag: bool) -> String {
        self.record.to_string(include_date, include_entry_tag)
    }

    pub fn created_deleted_record(&self) -> UiRecord {
        let input = self.record.to_deleted_string();
        UiRecord::parse(input, None).expect("deleted string should always be parsable")
    }

    pub fn day(&self) -> String {
        self.record.date_range().start().date().to_string()
    }

    pub fn revision(&self) -> usize {
        self.record.revision()
    }

    pub fn parse(input: String, override_revision: Option<usize>) -> Result<UiRecord, String> {
        let record = Record::parse(&input)?;
        let record = match override_revision {
            Some(revision) => record.with_updated_revision(revision),
            None => record,
        };
        Ok(UiRecord { record })
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct SkillData {
    pub title: String,
    pub kind: String,
    pub level: usize,
}

#[wasm_bindgen(getter_with_clone)]
pub struct SkillWeek {
    pub name: String,
    pub progress: usize,
    pub target: usize,
}

#[wasm_bindgen]
impl Views {
    pub fn new(onUpdate: js_sys::Function, onNotification: js_sys::Function) -> Self {
        let mut db = DB::default();
        db.on_view_update(Box::new(move |update| {
            let data = js_sys::Map::new();
            match update {
                ViewUpdate::QueryResults => {
                    data.set(&"view".into(), &"QueryResults".into());
                }
                ViewUpdate::Skills(update) => {
                    data.set(&"view".into(), &"Skills".into());
                    data.set(&"message".into(), &update.skill.into());
                }
                ViewUpdate::Week => {
                    data.set(&"view".into(), &"Week".into());
                }
            };
            if let Err(err) = onUpdate.call1(&JsValue::NULL, &data) {
                error(&err);
            }
        }));
        db.on_notification(Box::new(move |notification| {
            let data = js_sys::Map::new();
            match notification {
                Notification::Skills(SkillsNotification::HourProgress(msg)) => {
                    data.set(&"view".into(), &"Skills".into());
                    data.set(&"message".into(), &msg.into())
                }
                Notification::Skills(SkillsNotification::LevelUp(msg)) => {
                    data.set(&"view".into(), &"Skills".into());
                    data.set(&"message".into(), &msg.into())
                }
            };
            if let Err(err) = onNotification.call1(&JsValue::NULL, &data) {
                error(&err);
            }
        }));
        Self {
            db: RefCell::new(db),
        }
    }

    pub fn add_record(&self, record: &UiRecord, interactive: bool, now: Option<DateDay>) {
        let mut db = self.db.borrow_mut();
        db.add(record.record.clone(), interactive, now);
    }

    pub fn update_query(&self, query: String) -> Result<(), String> {
        let query = Query::new(&query).map_err(|v| v.to_string())?;
        let mut db = self.db.borrow_mut();
        db.update_query(query);
        Ok(())
    }

    pub fn query_results(&self) -> Vec<UiRecord> {
        let mut records = Vec::new();
        for record in self.db.borrow().query_results().iter() {
            let record = UiRecord {
                record: record.clone(),
            };
            records.push(record);
        }
        records
    }

    pub fn entry_count(&self) -> usize {
        self.db.borrow().count()
    }

    pub fn view_skills(&self) -> Vec<SkillData> {
        let db = self.db.borrow();
        let mut skills = db.skills().iter().map(|(_, v)| v).collect::<Vec<_>>();
        skills.sort();

        let mut output = Vec::new();
        for skill in skills {
            let skill_data = SkillData {
                title: skill.title().to_string(),
                kind: skill.kind().to_string(),
                level: skill.progress().level,
            };
            output.push(skill_data);
        }
        output
    }

    pub fn view_week(&self) -> Vec<SkillWeek> {
        let db = self.db.borrow();
        let mut output = Vec::new();
        for data in db.week().values() {
            let data = SkillWeek {
                name: data.skill().to_string(),
                progress: data.progress() as usize,
                target: data.target() as usize,
            };
            output.push(data);
        }
        output
    }
}
