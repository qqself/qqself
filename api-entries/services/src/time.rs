use std::{sync::Mutex, time::Duration};

use async_trait::async_trait;
use qqself_core::date_time::timestamp::Timestamp;

#[async_trait]
pub trait TimeProvider {
    async fn now(&self) -> Timestamp;
    async fn sleep(&self, duration: Duration);
}

/// StaticTime provides static pure timer. Sleep advances internal timestamp and returns immediately
pub struct TimeStatic(Mutex<u64>);

impl TimeStatic {
    pub fn new(milliseconds: u64) -> Self {
        Self(Mutex::from(milliseconds))
    }
}

#[async_trait]
impl TimeProvider for TimeStatic {
    async fn now(&self) -> Timestamp {
        let time = self.0.lock().unwrap();
        Timestamp::from_u64(*time)
    }
    async fn sleep(&self, duration: Duration) {
        let mut time = self.0.lock().unwrap();
        *time += duration.as_millis() as u64;
    }
}

/// Time provides access to real OS time
#[derive(Default)]
pub struct TimeOs();

#[async_trait]
impl TimeProvider for TimeOs {
    async fn now(&self) -> Timestamp {
        Timestamp::now()
    }
    async fn sleep(&self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }
}
