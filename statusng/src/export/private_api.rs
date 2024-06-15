use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::config::{BaseHistory, OutageConfig, ServiceStatus};
use crate::error::StatusError;
use crate::export::{ClientService, MiniHistory};
use crate::processors::ProcessorResult;

#[derive(Serialize, Debug)]
pub struct PrivateAPI {
    global: ServiceStatus,
    ping: f32,
    time: DateTime<Utc>,
    breakdown: MiniHistory,
    services: Vec<ClientService>,
    notice: Option<OutageConfig>
}

impl Default for PrivateAPI {
    fn default() -> Self {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();

        Self {
            global: ServiceStatus::Online,
            ping: 0.0,
            time: now,
            breakdown: MiniHistory(HashMap::new()),
            services: vec![],
            notice: None
        }
    }
}

impl PrivateAPI {
    pub fn new(outage: Option<OutageConfig>) -> Self {
        let mut api = Self::default();
        api.notice = outage;
        api
    }

    pub fn add(&mut self, item: &ProcessorResult) {
        self.services.push(item.into());
    }

    pub fn seal(&mut self, history: BaseHistory) {
        self.ping = self.services.iter()
            .map(|s| s.ping)
            .reduce(|a, b| a + b)
            .unwrap_or(0) as f32 / self.services.len() as f32;

        self.global = if self.services.iter()
            .find(|s| s.status == ServiceStatus::Offline).is_some() {
            ServiceStatus::Offline
        } else if self.services.iter()
            .find(|s| s.status == ServiceStatus::Unstable).is_some() {
            ServiceStatus::Unstable
        } else if self.services.iter()
            .find(|s| s.status == ServiceStatus::Maintenance).is_some() {
            ServiceStatus::Offline
        } else {
            ServiceStatus::Online
        };

        self.breakdown = MiniHistory::from_base(history);
    }

    pub fn sync(self) -> Result<(), StatusError> {
        fs::write("./out-private.json", serde_json::to_string(&self)?)?;
        Ok(())
    }
}
