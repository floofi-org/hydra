use std::fs;
use std::collections::HashMap;
use std::time::SystemTime;

use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::{
    History,
    service::{Service, ServiceStatus},
    config::OutageConfig,
};

use crate::error::StatusError;
use crate::models::service::ProcessorResult;

mod service;
mod breakdown;

use service::ClientService;
use breakdown::Breakdown;

#[derive(Serialize, Debug)]
pub struct PrivateAPI {
    global: ServiceStatus,
    ping: f32,
    time: DateTime<Utc>,
    breakdown: Breakdown,
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
            breakdown: Breakdown(HashMap::new()),
            services: vec![],
            notice: None
        }
    }
}

impl PrivateAPI {
    pub fn new(notice: Option<OutageConfig>) -> Self {
        Self {
            notice,
            ..Default::default()
        }
    }

    pub fn add(&mut self, service: &Service, item: &ProcessorResult) {
        self.services.push(ClientService::new(service, item.status, item.ping));
    }

    pub fn seal(&mut self, history: History) {
        self.ping = self.calc_average_ping();
        self.global = self.calc_global_status();
        self.breakdown = Breakdown::from_base(history);
    }

    fn calc_average_ping(&self) -> f32 {
        let total = self.services.iter()
            .map(|s| s.ping)
            .sum::<u32>() as f32;

        let count = self.services.len() as f32;

        total / count
    }

    fn calc_global_status(&self) -> ServiceStatus {
        let status = self.services.iter()
            .map(|s| s.status)
            .max()
            .unwrap_or_default();

        if let ServiceStatus::Maintenance = status {
            ServiceStatus::Offline
        } else {
            status
        }
    }

    pub fn sync(self) -> Result<(), StatusError> {
        fs::write("./out-private.json", serde_json::to_string(&self)?)?;
        Ok(())
    }
}
