use std::collections::BTreeMap;
use std::fs;
use std::time::SystemTime;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;

use crate::models::{
    config::OutageConfig,
    service::{Service, ServiceStatus},
    History,
};

use crate::error::StatusError;
use crate::export::private::breakdown::Breakdown;
use crate::export::private::service::ClientService;
use crate::export::vercel::Vercel;
use crate::models::service::ProcessorResult;

mod breakdown;
mod service;

#[derive(Serialize, Debug)]
pub struct PrivateAPI {
    pub global: ServiceStatus,
    pub ping: f32,
    time: DateTime<Utc>,
    breakdown: Breakdown,
    pub services: Vec<ClientService>,
    notice: Option<OutageConfig>,
}

impl Default for PrivateAPI {
    fn default() -> Self {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();

        Self {
            global: ServiceStatus::Online,
            ping: 0.0,
            time: now,
            breakdown: Breakdown(BTreeMap::new()),
            services: vec![],
            notice: None,
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
        self.services
            .push(ClientService::new(service, item.status, item.ping));
    }

    pub fn seal(&mut self, history: History) {
        self.ping = self.calc_average_ping();
        self.global = self.calc_global_status();
        self.breakdown = Breakdown::from_base(history);
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = vec![self.global as u8];
        bytes.append(&mut self.ping.to_le_bytes().to_vec());
        bytes.push(self.breakdown.0.len() as u8);

        for (date, statuses) in self.breakdown.0 {
            let date = NaiveDateTime::from(date);
            let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(date, Utc);
            let ts = date.timestamp() / 86400;

            bytes.append(&mut (ts as u32).to_le_bytes().to_vec());
            bytes.push(statuses.len() as u8);

            for status in statuses {
                bytes.append(&mut status.to_le_bytes().to_vec());
            }
        }

        bytes.push(self.services.len() as u8);
        for service in self.services {
            service.serialize(&mut bytes);
        }

        bytes.append(&mut (self.time.timestamp() as u64).to_le_bytes().to_vec());

        if let Some(notice) = self.notice {
            bytes.push(1);
            bytes.append(&mut serde_json::to_string(&notice).unwrap_or(String::from("{}")).into_bytes())
        } else {
            bytes.push(0);
        }

        bytes
    }

    fn calc_average_ping(&self) -> f32 {
        let total = self.services.iter().map(|s| s.ping).sum::<u32>() as f32;

        let count = self.services.len() as f32;

        total / count
    }

    fn calc_global_status(&self) -> ServiceStatus {
        let status = self
            .services
            .iter()
            .map(|s| s.status)
            .max()
            .unwrap_or_default();

        if let ServiceStatus::Maintenance = status {
            ServiceStatus::Offline
        } else {
            status
        }
    }

    pub fn sync(self, token: &str) -> Result<(), StatusError> {
        let data = self.into_bytes();
        let vercel = Vercel::new(token);

        fs::write("./output.dat", &data)?;
        vercel.put(&data, "public/status.dat", 360)?;

        Ok(())
    }
}
