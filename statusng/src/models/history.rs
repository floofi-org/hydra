use std::fs;
use std::collections::HashMap;
use std::time::SystemTime;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::StatusError;
use super::service::{Service, ServiceStatus};

type ServiceHistory = HashMap<NaiveDate, Vec<ServiceStatus>>;

fn get_current_date() -> NaiveDate {
    let now: DateTime<Utc> = SystemTime::now().into();
    let now: NaiveDate = now.date_naive();
    now
}

fn should_keep_entry(current_date: NaiveDate, entry_key: &NaiveDate) -> bool {
    let difference = current_date - *entry_key;
    difference.num_days() <= 90
}

#[derive(Deserialize, Serialize)]
pub struct History(pub HashMap<String, ServiceHistory>);

impl History {
    fn get_id_for_service(&self, service: &Service) -> String {
        // First check for legacy key format
        if self.0.contains_key(service.get_legacy_id()) {
            service.get_legacy_id().to_owned()
        } else {
            service.get_unique_id()
        }
    }

    pub fn add_entry(&mut self, service: &Service, code: ServiceStatus) {
        let id = self.get_id_for_service(service);

        let service = self.0.entry(id).or_default();
        let date = get_current_date();

        let statuses = service.entry(date).or_default();
        statuses.push(code);
    }

    pub fn vacuum(&mut self) {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now = now.date_naive();

        self.0.values_mut()
            .for_each(|service| service.retain(|date, _| should_keep_entry(now, date)));
    }

    pub fn sync(&self) -> Result<(), StatusError> {
        let text = serde_json::to_string(&self)?;
        fs::write("./history.json", text)?;
        Ok(())
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = vec![self.0.len() as u8];

        for (service, history) in self.0 {
            let mut service_bytes = service.into_bytes();

            bytes.push(service_bytes.len() as u8);
            bytes.append(&mut service_bytes);
            bytes.push(history.len() as u8);

            for (date, entries) in history {
                let date = NaiveDateTime::from(date);
                let date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(date, Utc);
                let ts = date.timestamp() / 86400;

                bytes.append(&mut (ts as u32).to_le_bytes().to_vec());
                bytes.append(&mut (entries.len() as u16).to_le_bytes().to_vec());
                bytes.append(&mut (entries.iter().map(|i| *i as u8).collect()))
            }
        }

        bytes
    }

    pub fn from_bytes(_bytes: &[u8]) -> Self {
        History(HashMap::new())
    }
}
